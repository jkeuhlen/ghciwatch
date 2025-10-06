window.BENCHMARK_DATA = {
  "lastUpdate": 1759783036692,
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
          "id": "024f665370cc2553761b499f136dcbcfcf8e0ca8",
          "message": "Create benchmarking tools (#4)\n\n- [ ] Labeled the PR with `patch`, `minor`, or `major` to request a\nversion bump when it's merged.\n- [ ] Updated the user manual in `docs/`.\n- [ ] Added integration / regression tests in `tests/`.",
          "timestamp": "2025-10-06T14:21:32-06:00",
          "tree_id": "b024a85e5c111c07b0a97b4b1f47e95ce1d7b0e2",
          "url": "https://github.com/jkeuhlen/ghciwatch/commit/024f665370cc2553761b499f136dcbcfcf8e0ca8"
        },
        "date": 1759783036659,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_ghc_messages/simple_error",
            "value": 403,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/multiple_errors",
            "value": 1160,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/warnings",
            "value": 1516,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/mixed_output",
            "value": 2031,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/large_output",
            "value": 29967,
            "range": "± 1405",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/ansi_heavy",
            "value": 1113,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/no_ansi",
            "value": 52920,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/light_ansi",
            "value": 35215,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/heavy_ansi",
            "value": 76090,
            "range": "± 685",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/parse_location",
            "value": 308,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/extract_context",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/typecheck_failure_cascade",
            "value": 9356,
            "range": "± 434",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/module_compilation_progress",
            "value": 6445,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/small",
            "value": 16231,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/medium",
            "value": 1117647,
            "range": "± 3954",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/large",
            "value": 16993913,
            "range": "± 78407",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/tiny",
            "value": 174,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/tiny",
            "value": 98,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/small",
            "value": 1589,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/small",
            "value": 1321,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/medium",
            "value": 23682,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/medium",
            "value": 20699,
            "range": "± 687",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/large",
            "value": 456315,
            "range": "± 1356",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/large",
            "value": 327843,
            "range": "± 1168",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/ascii",
            "value": 390301,
            "range": "± 3294",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/mixed_utf8",
            "value": 1429192,
            "range": "± 9946",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/emoji_heavy",
            "value": 1330092,
            "range": "± 7910",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_1",
            "value": 5302,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_3",
            "value": 15157,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_5",
            "value": 22799,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/ghci_session",
            "value": 149589,
            "range": "± 267",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/streaming_output",
            "value": 10021,
            "range": "± 65",
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
            "value": 2741,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/medium_batch",
            "value": 79680,
            "range": "± 459",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/large_batch",
            "value": 1517536,
            "range": "± 3580",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/deep_hierarchy",
            "value": 130901,
            "range": "± 4435",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_comparison",
            "value": 560636,
            "range": "± 6287",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_extension_check",
            "value": 31026,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_parent_extraction",
            "value": 44306,
            "range": "± 158",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/10",
            "value": 2494,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/10",
            "value": 2121,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/10",
            "value": 1961,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/100",
            "value": 59402,
            "range": "± 344",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/100",
            "value": 35116,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/100",
            "value": 68838,
            "range": "± 244",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/1000",
            "value": 971726,
            "range": "± 8133",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/1000",
            "value": 620066,
            "range": "± 4927",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/1000",
            "value": 1319433,
            "range": "± 4114",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/5000",
            "value": 6123752,
            "range": "± 12910",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/5000",
            "value": 3718678,
            "range": "± 20462",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/5000",
            "value": 8693170,
            "range": "± 34030",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/filter_haskell_files",
            "value": 98500,
            "range": "± 478",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/deduplicate_events",
            "value": 591570,
            "range": "± 1778",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/ide_save_burst",
            "value": 25555,
            "range": "± 200",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/git_checkout",
            "value": 33494,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/build_artifacts_ignore",
            "value": 272010,
            "range": "± 2883",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/no_commands",
            "value": 112,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/few_commands",
            "value": 3875,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/many_commands",
            "value": 11950,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/dense_commands",
            "value": 8271,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/small_file",
            "value": 9444,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/medium_file",
            "value": 146728,
            "range": "± 1837",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/large_file",
            "value": 1613199,
            "range": "± 55510",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/very_long_lines",
            "value": 123253,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/parse_continuation_lines",
            "value": 11807,
            "range": "± 490",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/extract_command_content",
            "value": 127,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/doctest_module",
            "value": 24203,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/mixed_eval_styles",
            "value": 13399,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/no_ansi",
            "value": 53052,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/light_ansi",
            "value": 38114,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/heavy_ansi",
            "value": 57793,
            "range": "± 320",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/mixed_content",
            "value": 49923,
            "range": "± 137",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/append_line/small",
            "value": 3608,
            "range": "± 8",
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
            "value": 35961,
            "range": "± 208",
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
            "value": 289252,
            "range": "± 740",
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
            "value": 11515,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/wrap_long_lines",
            "value": 3059,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/highlight_search",
            "value": 14754,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/progress_tracking",
            "value": 10134,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/format_durations",
            "value": 8434,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/full_compilation_render",
            "value": 52996,
            "range": "± 232",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/error_categorization",
            "value": 2018,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/rapid_updates",
            "value": 2455,
            "range": "± 9",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}