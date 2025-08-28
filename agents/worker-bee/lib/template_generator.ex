defmodule WorkerBee.TemplateGenerator do
  @moduledoc """
  Code scaffolding and template generation for WDD-compliant modules.
  
  Generates Elixir modules following Worker-Bee Driven Design principles
  based on the project's established structure and conventions.
  """

  alias WorkerBee.ProjectMapper

  @template_types [
    :functional_core,
    :boundary_genserver,
    :boundary_api,
    :data_struct,
    :test_functional,
    :test_boundary,
    :worker_process,
    :lifecycle_supervisor
  ]

  defstruct [
    :project_map,
    :template_type,
    :module_name,
    :target_path,
    :template_vars,
    :generated_content
  ]

  @type t :: %__MODULE__{
    project_map: ProjectMapper.t(),
    template_type: atom(),
    module_name: String.t(),
    target_path: String.t(),
    template_vars: map(),
    generated_content: String.t()
  }

  @doc """
  Scaffolds a new WDD component with all related files.
  """
  @spec scaffold_component(String.t(), String.t(), atom(), map()) :: {:ok, [String.t()]} | {:error, String.t()}
  def scaffold_component(project_path, component_name, component_type, options \\ %{}) do
    with {:ok, project_map} <- ProjectMapper.load_project_map(Path.join(project_path, ".wdd_project_map.yaml")),
         {:ok, files} <- generate_component_files(component_name, component_type, project_map, options) do
      
      created_files = Enum.map(files, fn {file_path, content} ->
        File.mkdir_p!(Path.dirname(file_path))
        File.write!(file_path, content)
        file_path
      end)
      
      {:ok, created_files}
    end
  end

  @doc """
  Generates a single template file.
  """
  @spec generate_template(atom(), String.t(), ProjectMapper.t(), map()) :: {:ok, t()} | {:error, String.t()}
  def generate_template(template_type, module_name, project_map, vars \\ %{}) do
    with {:ok, template_content} <- get_template_content(template_type),
         {:ok, target_path} <- determine_target_path(template_type, module_name, project_map),
         template_vars <- build_template_vars(module_name, project_map, vars),
         generated_content <- render_template(template_content, template_vars) do
      
      generator_result = %__MODULE__{
        project_map: project_map,
        template_type: template_type,
        module_name: module_name,
        target_path: target_path,
        template_vars: template_vars,
        generated_content: generated_content
      }
      
      {:ok, generator_result}
    end
  end

  @doc """
  Lists available template types.
  """
  def available_templates, do: @template_types

  @doc """
  Generates a complete functional core module.
  """
  def generate_functional_core(module_name, project_map, options \\ %{}) do
    template_vars = %{
      module_name: module_name,
      module_prefix: get_module_prefix(project_map),
      functions: Map.get(options, :functions, ["new/1", "update/2"]),
      data_types: Map.get(options, :data_types, []),
      with_specs: Map.get(options, :with_specs, true),
      with_docs: Map.get(options, :with_docs, true)
    }
    
    generate_template(:functional_core, module_name, project_map, template_vars)
  end

  @doc """
  Generates a boundary layer GenServer.
  """
  def generate_boundary_genserver(module_name, project_map, options \\ %{}) do
    template_vars = %{
      module_name: module_name,
      module_prefix: get_module_prefix(project_map),
      state_type: Map.get(options, :state_type, "map()"),
      api_functions: Map.get(options, :api_functions, ["start_link/1", "get_state/1"]),
      callbacks: Map.get(options, :callbacks, ["handle_call", "handle_cast"]),
      with_registry: Map.get(options, :with_registry, false)
    }
    
    generate_template(:boundary_genserver, module_name, project_map, template_vars)
  end

  @doc """
  Generates a data structure module.
  """
  def generate_data_struct(module_name, project_map, options \\ %{}) do
    template_vars = %{
      module_name: module_name,
      module_prefix: get_module_prefix(project_map),
      fields: Map.get(options, :fields, []),
      with_defaults: Map.get(options, :with_defaults, true),
      with_typespec: Map.get(options, :with_typespec, true),
      with_constructor: Map.get(options, :with_constructor, true)
    }
    
    generate_template(:data_struct, module_name, project_map, template_vars)
  end

  @doc """
  Generates test files for a given module.
  """
  def generate_tests(module_name, module_type, project_map, options \\ %{}) do
    template_type = case module_type do
      :functional_core -> :test_functional
      :boundary -> :test_boundary
      _ -> :test_functional
    end
    
    template_vars = %{
      module_name: module_name,
      module_prefix: get_module_prefix(project_map),
      test_type: module_type,
      with_describe_blocks: Map.get(options, :with_describe_blocks, true),
      with_setup: Map.get(options, :with_setup, false),
      test_functions: Map.get(options, :test_functions, [])
    }
    
    generate_template(template_type, "#{module_name}Test", project_map, template_vars)
  end

  # Private helper functions

  defp generate_component_files(component_name, component_type, project_map, options) do
    files = case component_type do
      :complete_wdd_component ->
        [
          generate_data_struct("#{component_name}Data", project_map, options),
          generate_functional_core("#{component_name}Core", project_map, options),
          generate_boundary_genserver("#{component_name}Server", project_map, options),
          generate_tests("#{component_name}Core", :functional_core, project_map, options),
          generate_tests("#{component_name}Server", :boundary, project_map, options)
        ]
      
      :functional_component ->
        [
          generate_functional_core(component_name, project_map, options),
          generate_tests(component_name, :functional_core, project_map, options)
        ]
      
      :boundary_component ->
        [
          generate_boundary_genserver(component_name, project_map, options),
          generate_tests(component_name, :boundary, project_map, options)
        ]
      
      _ ->
        [generate_template(component_type, component_name, project_map, options)]
    end
    
    # Resolve all file generation results
    resolved_files = Enum.reduce(files, [], fn
      {:ok, generator_result}, acc ->
        [{generator_result.target_path, generator_result.generated_content} | acc]
      
      {:error, _reason}, acc ->
        acc
    end)
    
    {:ok, Enum.reverse(resolved_files)}
  end

  defp get_template_content(template_type) do
    case template_type do
      :functional_core -> {:ok, functional_core_template()}
      :boundary_genserver -> {:ok, boundary_genserver_template()}
      :boundary_api -> {:ok, boundary_api_template()}
      :data_struct -> {:ok, data_struct_template()}
      :test_functional -> {:ok, test_functional_template()}
      :test_boundary -> {:ok, test_boundary_template()}
      :worker_process -> {:ok, worker_process_template()}
      :lifecycle_supervisor -> {:ok, lifecycle_supervisor_template()}
      _ -> {:error, "Unknown template type: #{template_type}"}
    end
  end

  defp determine_target_path(template_type, module_name, project_map) do
    layer_paths = project_map.layer_paths
    base_path = project_map.root_path
    
    relative_path = case template_type do
      :functional_core -> 
        Path.join([Map.get(layer_paths, :functions, "lib/core"), "#{Macro.underscore(module_name)}.ex"])
      
      :boundary_genserver -> 
        Path.join([Map.get(layer_paths, :boundaries, "lib/boundary"), "#{Macro.underscore(module_name)}.ex"])
      
      :boundary_api -> 
        Path.join([Map.get(layer_paths, :boundaries, "lib/boundary"), "#{Macro.underscore(module_name)}.ex"])
      
      :data_struct -> 
        Path.join([Map.get(layer_paths, :data, "lib/types"), "#{Macro.underscore(module_name)}.ex"])
      
      :test_functional -> 
        Path.join([Map.get(layer_paths, :tests, "test"), "#{Macro.underscore(module_name)}_test.exs"])
      
      :test_boundary -> 
        Path.join([Map.get(layer_paths, :tests, "test"), "#{Macro.underscore(module_name)}_test.exs"])
      
      :worker_process -> 
        Path.join([Map.get(layer_paths, :workers, "lib/workers"), "#{Macro.underscore(module_name)}.ex"])
      
      :lifecycle_supervisor -> 
        Path.join([Map.get(layer_paths, :lifecycles, "lib"), "#{Macro.underscore(module_name)}.ex"])
    end
    
    {:ok, Path.join(base_path, relative_path)}
  end

  defp build_template_vars(module_name, project_map, additional_vars) do
    base_vars = %{
      module_name: module_name,
      module_prefix: get_module_prefix(project_map),
      project_name: project_map.project_name,
      underscore_name: Macro.underscore(module_name),
      timestamp: DateTime.utc_now() |> DateTime.to_iso8601(),
      author: "Generated by Worker-Bee Agent"
    }
    
    Map.merge(base_vars, additional_vars)
  end

  defp render_template(template_content, vars) do
    Enum.reduce(vars, template_content, fn {key, value}, acc ->
      placeholder = "{{#{key}}}"
      String.replace(acc, placeholder, to_string(value))
    end)
  end

  defp get_module_prefix(project_map) do
    project_map.naming_conventions
    |> Map.get(:module_prefix, "")
    |> case do
      "" -> Macro.camelize(project_map.project_name)
      prefix -> prefix
    end
  end

  # Template definitions

  defp functional_core_template do
    """
    defmodule {{module_prefix}}.{{module_name}} do
      @moduledoc \"\"\"
      Functional core module for {{module_name}}.
      
      This module contains pure business logic without side effects,
      following Worker-Bee Driven Design principles.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      @type t :: %__MODULE__{}

      defstruct []

      @doc \"\"\"
      Creates a new {{underscore_name}}.
      \"\"\"
      @spec new(map()) :: {:ok, t()} | {:error, String.t()}
      def new(attrs \\\\ %{}) do
        # Implementation here
        {:ok, %__MODULE__{}}
      end

      @doc \"\"\"
      Updates a {{underscore_name}} with new attributes.
      \"\"\"
      @spec update(t(), map()) :: {:ok, t()} | {:error, String.t()}
      def update(%__MODULE__{} = {{underscore_name}}, attrs) do
        # Implementation here
        {:ok, {{underscore_name}}}
      end

      @doc \"\"\"
      Validates a {{underscore_name}}.
      \"\"\"
      @spec validate(t()) :: {:ok, t()} | {:error, String.t()}
      def validate(%__MODULE__{} = {{underscore_name}}) do
        # Validation logic here
        {:ok, {{underscore_name}}}
      end

      # Private helper functions

      defp do_something(data) do
        # Pure function implementation
        data
      end
    end
    """
  end

  defp boundary_genserver_template do
    """
    defmodule {{module_prefix}}.{{module_name}} do
      @moduledoc \"\"\"
      Boundary layer GenServer for {{module_name}}.
      
      This module manages state and side effects while delegating
      business logic to the functional core.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      use GenServer

      alias {{module_prefix}}.{{module_name}}Core

      @type state :: map()

      # Client API

      @doc \"\"\"
      Starts the {{module_name}} server.
      \"\"\"
      @spec start_link(keyword()) :: GenServer.on_start()
      def start_link(opts \\\\ []) do
        name = Keyword.get(opts, :name, __MODULE__)
        GenServer.start_link(__MODULE__, opts, name: name)
      end

      @doc \"\"\"
      Gets the current state.
      \"\"\"
      @spec get_state(GenServer.server()) :: state()
      def get_state(server \\\\ __MODULE__) do
        GenServer.call(server, :get_state)
      end

      @doc \"\"\"
      Performs an operation on the {{underscore_name}}.
      \"\"\"
      @spec perform_operation(GenServer.server(), term()) :: {:ok, term()} | {:error, String.t()}
      def perform_operation(server \\\\ __MODULE__, params) do
        GenServer.call(server, {:perform_operation, params})
      end

      # Server Callbacks

      @impl true
      def init(opts) do
        initial_state = %{
          # Initialize state here
        }
        
        {:ok, initial_state}
      end

      @impl true
      def handle_call(:get_state, _from, state) do
        {:reply, state, state}
      end

      @impl true
      def handle_call({:perform_operation, params}, _from, state) do
        with {:ok, result} <- {{module_name}}Core.perform_operation(params) do
          new_state = update_state(state, result)
          {:reply, {:ok, result}, new_state}
        else
          {:error, reason} ->
            {:reply, {:error, reason}, state}
        end
      end

      @impl true
      def handle_cast({:async_operation, params}, state) do
        # Handle async operations
        {:noreply, state}
      end

      @impl true
      def handle_info(msg, state) do
        # Handle info messages
        {:noreply, state}
      end

      # Private helper functions

      defp update_state(state, _result) do
        # State update logic
        state
      end
    end
    """
  end

  defp boundary_api_template do
    """
    defmodule {{module_prefix}}.{{module_name}} do
      @moduledoc \"\"\"
      API boundary for {{module_name}}.
      
      This module provides a clean API interface that handles
      validation and delegates to the functional core.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      alias {{module_prefix}}.{{module_name}}Core

      @doc \"\"\"
      Creates a new {{underscore_name}}.
      \"\"\"
      @spec create(map()) :: {:ok, term()} | {:error, String.t()}
      def create(attrs) do
        with {:ok, validated_attrs} <- validate_attrs(attrs),
             {:ok, result} <- {{module_name}}Core.create(validated_attrs) do
          {:ok, result}
        end
      end

      @doc \"\"\"
      Updates an existing {{underscore_name}}.
      \"\"\"
      @spec update(String.t(), map()) :: {:ok, term()} | {:error, String.t()}
      def update(id, attrs) do
        with {:ok, validated_id} <- validate_id(id),
             {:ok, validated_attrs} <- validate_attrs(attrs),
             {:ok, result} <- {{module_name}}Core.update(validated_id, validated_attrs) do
          {:ok, result}
        end
      end

      @doc \"\"\"
      Retrieves a {{underscore_name}} by ID.
      \"\"\"
      @spec get(String.t()) :: {:ok, term()} | {:error, String.t()}
      def get(id) do
        with {:ok, validated_id} <- validate_id(id) do
          {{module_name}}Core.get(validated_id)
        end
      end

      # Private validation functions

      defp validate_attrs(attrs) when is_map(attrs) do
        # Validation logic here
        {:ok, attrs}
      end
      
      defp validate_attrs(_), do: {:error, "Invalid attributes format"}

      defp validate_id(id) when is_binary(id) and id != "" do
        {:ok, id}
      end
      
      defp validate_id(_), do: {:error, "Invalid ID format"}
    end
    """
  end

  defp data_struct_template do
    """
    defmodule {{module_prefix}}.{{module_name}} do
      @moduledoc \"\"\"
      Data structure for {{module_name}}.
      
      This module defines the core data structure and related
      functions following Worker-Bee Driven Design principles.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      @type t :: %__MODULE__{
        id: String.t() | nil,
        name: String.t(),
        created_at: DateTime.t(),
        updated_at: DateTime.t()
      }

      defstruct [
        :id,
        :name,
        created_at: nil,
        updated_at: nil
      ]

      @doc \"\"\"
      Creates a new {{underscore_name}} struct.
      \"\"\"
      @spec new(map()) :: t()
      def new(attrs \\\\ %{}) do
        now = DateTime.utc_now()
        
        %__MODULE__{
          id: Map.get(attrs, :id),
          name: Map.get(attrs, :name, ""),
          created_at: Map.get(attrs, :created_at, now),
          updated_at: Map.get(attrs, :updated_at, now)
        }
      end

      @doc \"\"\"
      Updates a {{underscore_name}} struct with new attributes.
      \"\"\"
      @spec update(t(), map()) :: t()
      def update(%__MODULE__{} = {{underscore_name}}, attrs) do
        updated_attrs = Map.put(attrs, :updated_at, DateTime.utc_now())
        struct({{underscore_name}}, updated_attrs)
      end

      @doc \"\"\"
      Validates a {{underscore_name}} struct.
      \"\"\"
      @spec valid?(t()) :: boolean()
      def valid?(%__MODULE__{name: name}) when is_binary(name) and name != "" do
        true
      end
      
      def valid?(_), do: false
    end
    """
  end

  defp test_functional_template do
    """
    defmodule {{module_prefix}}.{{module_name}}Test do
      @moduledoc \"\"\"
      Tests for {{module_name}} functional core.
      
      These tests focus on behavior and business logic validation
      without side effects or process machinery.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      use ExUnit.Case, async: true

      alias {{module_prefix}}.{{module_name}}

      describe "{{underscore_name}}/0" do
        test "creates a new {{underscore_name}} with default values" do
          result = {{module_name}}.new()
          
          assert {:ok, {{underscore_name}}} = result
          assert %{{module_name}}{} = {{underscore_name}}
        end

        test "creates a new {{underscore_name}} with provided attributes" do
          attrs = %{name: "test {{underscore_name}}"}
          
          result = {{module_name}}.new(attrs)
          
          assert {:ok, {{underscore_name}}} = result
          assert {{underscore_name}}.name == "test {{underscore_name}}"
        end
      end

      describe "update/2" do
        test "updates {{underscore_name}} with new attributes" do
          {:ok, {{underscore_name}}} = {{module_name}}.new(%{name: "original"})
          
          result = {{module_name}}.update({{underscore_name}}, %{name: "updated"})
          
          assert {:ok, updated_{{underscore_name}}} = result
          assert updated_{{underscore_name}}.name == "updated"
        end

        test "returns error for invalid attributes" do
          {:ok, {{underscore_name}}} = {{module_name}}.new()
          
          result = {{module_name}}.update({{underscore_name}}, %{invalid: "attr"})
          
          assert {:error, _reason} = result
        end
      end

      describe "validate/1" do
        test "validates a valid {{underscore_name}}" do
          {:ok, {{underscore_name}}} = {{module_name}}.new(%{name: "valid"})
          
          result = {{module_name}}.validate({{underscore_name}})
          
          assert {:ok, ^{{underscore_name}}} = result
        end

        test "returns error for invalid {{underscore_name}}" do
          {:ok, {{underscore_name}}} = {{module_name}}.new(%{name: ""})
          
          result = {{module_name}}.validate({{underscore_name}})
          
          assert {:error, _reason} = result
        end
      end

      # Helper functions for test data

      defp valid_{{underscore_name}}_attrs do
        %{
          name: "Test {{module_name}}"
        }
      end

      defp invalid_{{underscore_name}}_attrs do
        %{
          name: ""
        }
      end
    end
    """
  end

  defp test_boundary_template do
    """
    defmodule {{module_prefix}}.{{module_name}}Test do
      @moduledoc \"\"\"
      Integration tests for {{module_name}} boundary layer.
      
      These tests exercise the process behavior and API
      interactions of the boundary layer.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      use ExUnit.Case, async: true

      alias {{module_prefix}}.{{module_name}}

      setup do
        {:ok, pid} = {{module_name}}.start_link()
        %{server: pid}
      end

      describe "start_link/1" do
        test "starts the server successfully" do
          assert {:ok, pid} = {{module_name}}.start_link()
          assert Process.alive?(pid)
        end

        test "can start named server" do
          assert {:ok, _pid} = {{module_name}}.start_link(name: :test_server)
          assert Process.whereis(:test_server)
        end
      end

      describe "get_state/1" do
        test "returns current server state", %{server: server} do
          state = {{module_name}}.get_state(server)
          
          assert is_map(state)
        end
      end

      describe "perform_operation/2" do
        test "performs operation successfully", %{server: server} do
          params = %{action: "test"}
          
          result = {{module_name}}.perform_operation(server, params)
          
          assert {:ok, _result} = result
        end

        test "handles invalid parameters", %{server: server} do
          invalid_params = %{invalid: "params"}
          
          result = {{module_name}}.perform_operation(server, invalid_params)
          
          assert {:error, _reason} = result
        end

        test "maintains state consistency", %{server: server} do
          initial_state = {{module_name}}.get_state(server)
          
          {{module_name}}.perform_operation(server, %{action: "test"})
          
          final_state = {{module_name}}.get_state(server)
          
          # Assert state changes as expected
          refute initial_state == final_state
        end
      end

      # Helper functions for test data

      defp valid_operation_params do
        %{
          action: "test_action",
          data: %{key: "value"}
        }
      end

      defp invalid_operation_params do
        %{
          invalid: "parameters"
        }
      end
    end
    """
  end

  defp worker_process_template do
    """
    defmodule {{module_prefix}}.{{module_name}} do
      @moduledoc \"\"\"
      Worker process for {{module_name}}.
      
      This module handles concurrent work and background processing
      following Worker-Bee Driven Design principles.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      use GenServer

      alias {{module_prefix}}.{{module_name}}Core

      @type state :: %{
        queue: [term()],
        processing: boolean(),
        results: [term()]
      }

      # Client API

      @doc \"\"\"
      Starts the worker process.
      \"\"\"
      @spec start_link(keyword()) :: GenServer.on_start()
      def start_link(opts \\\\ []) do
        name = Keyword.get(opts, :name, __MODULE__)
        GenServer.start_link(__MODULE__, opts, name: name)
      end

      @doc \"\"\"
      Adds work to the queue.
      \"\"\"
      @spec add_work(GenServer.server(), term()) :: :ok
      def add_work(server \\\\ __MODULE__, work_item) do
        GenServer.cast(server, {:add_work, work_item})
      end

      @doc \"\"\"
      Gets the current status of the worker.
      \"\"\"
      @spec get_status(GenServer.server()) :: map()
      def get_status(server \\\\ __MODULE__) do
        GenServer.call(server, :get_status)
      end

      # Server Callbacks

      @impl true
      def init(_opts) do
        state = %{
          queue: [],
          processing: false,
          results: []
        }
        
        {:ok, state}
      end

      @impl true
      def handle_call(:get_status, _from, state) do
        status = %{
          queue_length: length(state.queue),
          processing: state.processing,
          results_count: length(state.results)
        }
        
        {:reply, status, state}
      end

      @impl true
      def handle_cast({:add_work, work_item}, state) do
        new_queue = state.queue ++ [work_item]
        new_state = %{state | queue: new_queue}
        
        # Start processing if not already processing
        if not state.processing do
          send(self(), :process_next)
        end
        
        {:noreply, new_state}
      end

      @impl true
      def handle_info(:process_next, %{queue: []} = state) do
        # No work to process
        {:noreply, %{state | processing: false}}
      end

      @impl true
      def handle_info(:process_next, %{queue: [work_item | rest]} = state) do
        # Process work item using functional core
        result = {{module_name}}Core.process_work(work_item)
        
        new_state = %{
          state |
          queue: rest,
          processing: true,
          results: [result | state.results]
        }
        
        # Continue processing if more work exists
        if not Enum.empty?(rest) do
          send(self(), :process_next)
        else
          new_state = %{new_state | processing: false}
        end
        
        {:noreply, new_state}
      end
    end
    """
  end

  defp lifecycle_supervisor_template do
    """
    defmodule {{module_prefix}}.{{module_name}} do
      @moduledoc \"\"\"
      Supervisor for {{module_name}} lifecycle management.
      
      This module manages the lifecycle of child processes
      following OTP supervision principles.
      
      Generated by Worker-Bee Agent on {{timestamp}}
      \"\"\"

      use Supervisor

      @doc \"\"\"
      Starts the supervisor.
      \"\"\"
      @spec start_link(keyword()) :: Supervisor.on_start()
      def start_link(opts \\\\ []) do
        name = Keyword.get(opts, :name, __MODULE__)
        Supervisor.start_link(__MODULE__, opts, name: name)
      end

      @impl true
      def init(_opts) do
        children = [
          # Define child processes here
          # {{{module_prefix}}.SomeServer, []},
          # {{{module_prefix}}.SomeWorker, []}
        ]

        opts = [strategy: :one_for_one, name: __MODULE__]
        Supervisor.init(children, opts)
      end

      @doc \"\"\"
      Dynamically starts a child process.
      \"\"\"
      @spec start_child(module(), term()) :: Supervisor.on_start_child()
      def start_child(module, args) do
        child_spec = {module, args}
        Supervisor.start_child(__MODULE__, child_spec)
      end

      @doc \"\"\"
      Stops a child process.
      \"\"\"
      @spec stop_child(pid()) :: :ok | {:error, term()}
      def stop_child(pid) when is_pid(pid) do
        Supervisor.terminate_child(__MODULE__, pid)
      end

      @doc \"\"\"
      Lists all child processes.
      \"\"\"
      @spec list_children() :: [Supervisor.child()]
      def list_children do
        Supervisor.which_children(__MODULE__)
      end
    end
    """
  end
end