window.BENCHMARK_DATA = {
  "lastUpdate": 1760046405450,
  "repoUrl": "https://github.com/jkeuhlen/ghciwatch",
  "entries": {
    "ghciwatch Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "jak1214@gmail.com",
            "name": "Jake Keuhlen",
            "username": "jkeuhlen"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7269b9f4b9a1ea3b0aec66ee284fcd92aa8ed4f3",
          "message": "Upgrade the notifier library (#12)\n\n- [ ] Labeled the PR with `patch`, `minor`, or `major` to request a\nversion bump when it's merged.\n- [ ] Updated the user manual in `docs/`.\n- [ ] Added integration / regression tests in `tests/`.",
          "timestamp": "2025-10-09T15:30:56-06:00",
          "tree_id": "f8d71996858937c9156d7a6dd8abae10fc3051b2",
          "url": "https://github.com/jkeuhlen/ghciwatch/commit/7269b9f4b9a1ea3b0aec66ee284fcd92aa8ed4f3"
        },
        "date": 1760046405416,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_ghc_messages/simple_error",
            "value": 406,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/multiple_errors",
            "value": 1130,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/warnings",
            "value": 1486,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/mixed_output",
            "value": 1936,
            "range": "± 76",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/large_output",
            "value": 28052,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/ansi_heavy",
            "value": 1085,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/no_ansi",
            "value": 52934,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/light_ansi",
            "value": 34217,
            "range": "± 213",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/heavy_ansi",
            "value": 75747,
            "range": "± 420",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/parse_location",
            "value": 307,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/extract_context",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/typecheck_failure_cascade",
            "value": 8961,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/module_compilation_progress",
            "value": 6196,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/small",
            "value": 16266,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/medium",
            "value": 1117768,
            "range": "± 8035",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/large",
            "value": 17052421,
            "range": "± 25123",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/tiny",
            "value": 175,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/tiny",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/small",
            "value": 1591,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/small",
            "value": 1638,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/medium",
            "value": 23107,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/medium",
            "value": 25538,
            "range": "± 1574",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/large",
            "value": 456330,
            "range": "± 706",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/large",
            "value": 408921,
            "range": "± 1084",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/ascii",
            "value": 290996,
            "range": "± 498",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/mixed_utf8",
            "value": 1395001,
            "range": "± 20112",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/emoji_heavy",
            "value": 1377481,
            "range": "± 15180",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_1",
            "value": 5130,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_3",
            "value": 15063,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_5",
            "value": 22727,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/ghci_session",
            "value": 148722,
            "range": "± 13641",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/streaming_output",
            "value": 11977,
            "range": "± 503",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/single_file",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/small_batch",
            "value": 2812,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/medium_batch",
            "value": 80680,
            "range": "± 463",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/large_batch",
            "value": 1521316,
            "range": "± 3571",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/deep_hierarchy",
            "value": 132359,
            "range": "± 566",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_comparison",
            "value": 570421,
            "range": "± 3218",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_extension_check",
            "value": 31344,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_parent_extraction",
            "value": 43827,
            "range": "± 117",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/10",
            "value": 2497,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/10",
            "value": 2097,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/10",
            "value": 1998,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/100",
            "value": 59879,
            "range": "± 140",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/100",
            "value": 35177,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/100",
            "value": 69970,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/1000",
            "value": 979027,
            "range": "± 21680",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/1000",
            "value": 623733,
            "range": "± 5320",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/1000",
            "value": 1328131,
            "range": "± 6408",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/5000",
            "value": 6171237,
            "range": "± 16095",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/5000",
            "value": 3724980,
            "range": "± 44504",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/5000",
            "value": 8737349,
            "range": "± 537188",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/filter_haskell_files",
            "value": 98349,
            "range": "± 1618",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/deduplicate_events",
            "value": 595875,
            "range": "± 11102",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/ide_save_burst",
            "value": 25797,
            "range": "± 470",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/git_checkout",
            "value": 33545,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/build_artifacts_ignore",
            "value": 279659,
            "range": "± 2226",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/no_commands",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/few_commands",
            "value": 3477,
            "range": "± 54",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/many_commands",
            "value": 11027,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/dense_commands",
            "value": 7636,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/small_file",
            "value": 9370,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/medium_file",
            "value": 143300,
            "range": "± 667",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/large_file",
            "value": 1601260,
            "range": "± 2925",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/very_long_lines",
            "value": 123610,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/parse_continuation_lines",
            "value": 11256,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/extract_command_content",
            "value": 128,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/doctest_module",
            "value": 23938,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/mixed_eval_styles",
            "value": 13224,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/no_ansi",
            "value": 53060,
            "range": "± 252",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/light_ansi",
            "value": 37584,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/heavy_ansi",
            "value": 57607,
            "range": "± 303",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/mixed_content",
            "value": 50149,
            "range": "± 2820",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/append_line/small",
            "value": 3598,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/scroll_viewport/small",
            "value": 343,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/append_line/medium",
            "value": 35826,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/scroll_viewport/medium",
            "value": 343,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/append_line/large",
            "value": 319388,
            "range": "± 1438",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/scroll_viewport/large",
            "value": 343,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/format_error_lines",
            "value": 14371,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/wrap_long_lines",
            "value": 3132,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/highlight_search",
            "value": 15065,
            "range": "± 80",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/progress_tracking",
            "value": 10687,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/format_durations",
            "value": 8509,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/full_compilation_render",
            "value": 54392,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/error_categorization",
            "value": 1946,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/rapid_updates",
            "value": 2474,
            "range": "± 8",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}