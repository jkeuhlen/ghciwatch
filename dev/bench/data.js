window.BENCHMARK_DATA = {
  "lastUpdate": 1760730032217,
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
          "id": "c1dd40338d77af4cd054ddf50a8443a78c4be033",
          "message": "Fix copy/paste out of TUI mode (#20)\n\n- [ ] Labeled the PR with `patch`, `minor`, or `major` to request a\nversion bump when it's merged.\n- [ ] Updated the user manual in `docs/`.\n- [ ] Added integration / regression tests in `tests/`.",
          "timestamp": "2025-10-17T13:22:59-06:00",
          "tree_id": "c2aae2ed661bb10476159746edd7213399a47354",
          "url": "https://github.com/jkeuhlen/ghciwatch/commit/c1dd40338d77af4cd054ddf50a8443a78c4be033"
        },
        "date": 1760730032184,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_ghc_messages/simple_error",
            "value": 405,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/multiple_errors",
            "value": 1166,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/warnings",
            "value": 1500,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/mixed_output",
            "value": 2009,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/large_output",
            "value": 29492,
            "range": "± 272",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/ansi_heavy",
            "value": 1093,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/no_ansi",
            "value": 51752,
            "range": "± 301",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/light_ansi",
            "value": 33349,
            "range": "± 1528",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/heavy_ansi",
            "value": 74396,
            "range": "± 734",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/parse_location",
            "value": 303,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/extract_context",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/typecheck_failure_cascade",
            "value": 8752,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/module_compilation_progress",
            "value": 6075,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/small",
            "value": 16309,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/medium",
            "value": 1111550,
            "range": "± 7087",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/large",
            "value": 17081107,
            "range": "± 29881",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/tiny",
            "value": 166,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/tiny",
            "value": 97,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/small",
            "value": 1538,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/small",
            "value": 1325,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/medium",
            "value": 22264,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/medium",
            "value": 20466,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/large",
            "value": 459539,
            "range": "± 1879",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/large",
            "value": 328003,
            "range": "± 2006",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/ascii",
            "value": 314170,
            "range": "± 3548",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/mixed_utf8",
            "value": 1277118,
            "range": "± 10091",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/emoji_heavy",
            "value": 1633194,
            "range": "± 5686",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_1",
            "value": 5367,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_3",
            "value": 15097,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_5",
            "value": 23283,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/ghci_session",
            "value": 150002,
            "range": "± 4160",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/streaming_output",
            "value": 10996,
            "range": "± 93",
            "unit": "ns/iter"
          },
          {
            "name": "real_incremental_read_ansi/hspec_session_with_ansi",
            "value": 5359,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/no_ansi",
            "value": 6500,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/no_ansi",
            "value": 661,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/light_ansi",
            "value": 7403,
            "range": "± 625",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/light_ansi",
            "value": 753,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/ansi_at_end",
            "value": 4088,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/ansi_at_end",
            "value": 427,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/hspec_case",
            "value": 1902,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/hspec_case",
            "value": 210,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/heavy_ansi",
            "value": 5172,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/heavy_ansi",
            "value": 547,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/single_file",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/small_batch",
            "value": 2749,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/medium_batch",
            "value": 79727,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/large_batch",
            "value": 1520929,
            "range": "± 7082",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/deep_hierarchy",
            "value": 130797,
            "range": "± 622",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_comparison",
            "value": 604551,
            "range": "± 9590",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_extension_check",
            "value": 29176,
            "range": "± 169",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_parent_extraction",
            "value": 43391,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/10",
            "value": 2481,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/10",
            "value": 2183,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/10",
            "value": 1967,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/100",
            "value": 57769,
            "range": "± 310",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/100",
            "value": 35881,
            "range": "± 208",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/100",
            "value": 68362,
            "range": "± 683",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/1000",
            "value": 967502,
            "range": "± 28404",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/1000",
            "value": 635022,
            "range": "± 28351",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/1000",
            "value": 1302226,
            "range": "± 43642",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/5000",
            "value": 6093140,
            "range": "± 18324",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/5000",
            "value": 3792374,
            "range": "± 24135",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/5000",
            "value": 8600099,
            "range": "± 29312",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/filter_haskell_files",
            "value": 100546,
            "range": "± 3447",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/deduplicate_events",
            "value": 594169,
            "range": "± 4819",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/ide_save_burst",
            "value": 25221,
            "range": "± 132",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/git_checkout",
            "value": 33554,
            "range": "± 196",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/build_artifacts_ignore",
            "value": 277827,
            "range": "± 2799",
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
            "value": 3507,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/many_commands",
            "value": 10992,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/dense_commands",
            "value": 7558,
            "range": "± 235",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/small_file",
            "value": 9336,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/medium_file",
            "value": 147959,
            "range": "± 1026",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/large_file",
            "value": 1609106,
            "range": "± 73365",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/very_long_lines",
            "value": 120254,
            "range": "± 850",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/parse_continuation_lines",
            "value": 12557,
            "range": "± 411",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/extract_command_content",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/doctest_module",
            "value": 25090,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/mixed_eval_styles",
            "value": 13479,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/no_ansi",
            "value": 54182,
            "range": "± 1751",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/light_ansi",
            "value": 38807,
            "range": "± 402",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/heavy_ansi",
            "value": 58784,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/mixed_content",
            "value": 51880,
            "range": "± 1275",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/append_line/small",
            "value": 3545,
            "range": "± 28",
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
            "value": 35897,
            "range": "± 418",
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
            "value": 314917,
            "range": "± 1568",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/scroll_viewport/large",
            "value": 343,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/format_error_lines",
            "value": 12049,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/wrap_long_lines",
            "value": 3295,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/highlight_search",
            "value": 13973,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/progress_tracking",
            "value": 10227,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/format_durations",
            "value": 8397,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/full_compilation_render",
            "value": 54053,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/error_categorization",
            "value": 2152,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/rapid_updates",
            "value": 2379,
            "range": "± 31",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}