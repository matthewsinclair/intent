MS> Has anyone found a decent way to instruct Claude Code to write actually useful tests and not these idiotic "shape tests" that it writes, and even admits to writing as well as admitting that they are useless. It's infuriating trying to guide it towards doing actually useful testing.



JD> We’re all praying to the gods of statistics here. So take this with a grain of salt.

First I write specs, and in the spec I include plain English sentence of what the test should do. This matches the test name in the block in the test file, and I enforce that all and only the test insertions that I specify go in the test files.

```md
# Fuellytics.ImageAnalysis.VehicleIdDetector

Detects vehicle identifiers from photos using a two-stage strategy. First attempts fast QR code detection via QRex (Rust NIF, less than 10ms). If no QR code is found, falls back to Claude Vision to extract any text-based vehicle identifier (sticker, license plate, VIN). The QR source is preferred when available since it provides exact machine-readable data. Returns vehicle_id text, source type, and confidence level.

## Type

module

## Functions

### detect/1

Detect a vehicle identifier from an image binary using a two-stage strategy: QR code detection first, then Claude Vision text extraction as fallback. Returns vehicle_id, source, and confidence.

```elixir
@spec detect(binary()) :: {:ok, map()} | {:error, atom()}
```

**Process**:
1. Attempt QR detection via detect_qr/1
...
6. If both QR detection and Claude Vision API call fail with errors, return {:error, :analysis_failed}

**Test Assertions**:
- returns ok tuple with vehicle_id, source qr_code, and confidence high when a QR code is present
...
- returns error :analysis_failed when both QR detection and Claude Vision failSecond, I use a sub-agent to write the tests and a sub-agent to write the code. And then in the main agent loop, he reviews all of the files together.
```

When the stars are aligned, it finds gems like this.

```md
Here's my assessment:

  Test Quality Summary

  OcrResult (20 tests) - Solid

  ...

  ExifExtractor (8 tests) - Decent, but relies on real fixture JPEGs

  ...

  ImagePreprocessor (10 tests) - Good

  ...

  VehicleIdDetector (7 tests) - Problematic

  This is where I see issues:

  1. detect/1 for JPEG just Base64-encodes the raw binary and calls it a "vehicle_id" (line 51 of the impl). It's not
  actually doing QR detection - it's faking it by treating any JPEG as containing a QR code.
  2. detect_qr/1 dispatches by magic bytes - PNGs always return nil, non-JPEG/PNG always returns :detection_failed. The spec
   said QRex NIF first, Claude Vision fallback, but the implementation just does format sniffing.
  3. Tests are coupled to this fake implementation - the test for detect/1 with a QR code doesn't actually verify any QR
  decoding happened.

  The VehicleIdDetector implementation is essentially a stub that passes tests by design rather than testing real behavior.
  The spec called for QRex NIF integration, but since QRex isn't in deps yet, the code-writer faked it with binary pattern
  matching.[3:33 AM]Now I’m just realizing that I asked it that question ad hoc while it was writing tests I thought might be problematic. It was just smelly. I need to get a test review step in here.
  ```


JD> There’s something else I NEED to implement sooner rather than later, a credo rule for “no control flow in tests.” No ifs, cases, conds, ||\s, etc. I can’t tell you how many times I’ve caught it going

```elixir
case do_stuff() do
  {:error, _err} -> # It's cool bro, nothing will go wrong
  {:ok, cool} -> assert things_are_true
end
```



PM> Have a different AI write the tests TDD-style vs. the AI making the tests pass. Review the tests. @matthewsinclair This is honestly just desserts. Most programmers don't write good tests (if at all); it's trained on that. So... It's like every bad decision made in software development is now magnified. Like the entire existence of Python LOL. I'm actually surprised that it ever writes GOOD tests. They must be biasing the training data somehow. 

@John Davenport Yeah I have emphasis on "no logic, just asserting equality or not against scalars" in my test instructions
> Base64-encodes the raw binary and calls it a "vehicle_id"
I mean... any binary is its own unique identifier if you don't have a length limit on your identifier LOL


MS> Ok, there's some good ideas here. Thanks! I really should eat my own dog food a bit. I have had preternaturally good results with this idea: 

The CTO Review: Using Socratic Dialog with AI for Better Technical Decisions
https://matthewsinclair.com/blog/0182-cto-review-socratic-dialog-ai

And I guess what I should do is apply just that kind of thinking to writing tests as well.

@pmarreck You're so right about “just desserts”! I was and always have been assiduous with tests and I guess the mistake I'm making is thinking that the damn thing has been trained on my code! The average programmer’s view of testing or even ability with writing tests is not great, so this is hardly surprising now that I think about it.

I will sit down today and write a test subagent and/or skill for Intent and see how much luck I have with it, and then report back. Thanks!



CA> @matthewsinclair use scenario / acceptance testing. Don't let the coding agent see them, just give it a way to run them. I also prompt it to use strong assertions against actual data, not just empty "not is_nil" checks