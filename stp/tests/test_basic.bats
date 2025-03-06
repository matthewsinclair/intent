#!/usr/bin/env bats
# Basic test to check if Bats is working correctly

@test "Check if true command works" {
  run true
  [ "$status" -eq 0 ]
}

@test "Check if echo works" {
  result="$(echo 'Hello, World!')"
  [ "$result" == "Hello, World!" ]
}

@test "Check if test variables work" {
  value="example"
  [ "$value" == "example" ]
}