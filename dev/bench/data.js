window.BENCHMARK_DATA = {
  "lastUpdate": 1760131752258,
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
          "id": "85ade2807c3631f7983b9fcb1514b1ecc5e273d5",
          "message": "incremental reader optimizations (#17)\n\n- [ ] Labeled the PR with `patch`, `minor`, or `major` to request a\nversion bump when it's merged.\n- [ ] Updated the user manual in `docs/`.\n- [ ] Added integration / regression tests in `tests/`.",
          "timestamp": "2025-10-10T15:12:01-06:00",
          "tree_id": "e748cf79b6deb9c40b3e66859c0ce07e8c41e02f",
          "url": "https://github.com/jkeuhlen/ghciwatch/commit/85ade2807c3631f7983b9fcb1514b1ecc5e273d5"
        },
        "date": 1760131752223,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_ghc_messages/simple_error",
            "value": 389,
            "range": "± 3",
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
            "value": 1492,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/mixed_output",
            "value": 1940,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/large_output",
            "value": 28059,
            "range": "± 977",
            "unit": "ns/iter"
          },
          {
            "name": "parse_ghc_messages/ansi_heavy",
            "value": 1089,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/no_ansi",
            "value": 52981,
            "range": "± 851",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/light_ansi",
            "value": 34284,
            "range": "± 645",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping/heavy_ansi",
            "value": 75776,
            "range": "± 3855",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/parse_location",
            "value": 307,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "line_parsing/extract_context",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/typecheck_failure_cascade",
            "value": 8993,
            "range": "± 502",
            "unit": "ns/iter"
          },
          {
            "name": "real_world_parsing/module_compilation_progress",
            "value": 6194,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/small",
            "value": 16398,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/medium",
            "value": 1129705,
            "range": "± 17586",
            "unit": "ns/iter"
          },
          {
            "name": "stream_processing/large",
            "value": 18318383,
            "range": "± 17044",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/tiny",
            "value": 181,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/tiny",
            "value": 98,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/small",
            "value": 1657,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/small",
            "value": 1325,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/medium",
            "value": 24534,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/medium",
            "value": 20460,
            "range": "± 625",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/consume/large",
            "value": 473559,
            "range": "± 2258",
            "unit": "ns/iter"
          },
          {
            "name": "buffer_operations/ring_buffer/large",
            "value": 328143,
            "range": "± 2585",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/ascii",
            "value": 339752,
            "range": "± 1571",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/mixed_utf8",
            "value": 1437321,
            "range": "± 5562",
            "unit": "ns/iter"
          },
          {
            "name": "utf8_handling/emoji_heavy",
            "value": 1501441,
            "range": "± 9485",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_1",
            "value": 5350,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_3",
            "value": 15177,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "pattern_searching/patterns_5",
            "value": 22861,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/ghci_session",
            "value": 149545,
            "range": "± 657",
            "unit": "ns/iter"
          },
          {
            "name": "incremental_reader_scenarios/streaming_output",
            "value": 9880,
            "range": "± 147",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/no_ansi",
            "value": 6201,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/no_ansi",
            "value": 661,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/light_ansi",
            "value": 7280,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/light_ansi",
            "value": 761,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/ansi_at_end",
            "value": 3926,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/ansi_at_end",
            "value": 414,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/hspec_case",
            "value": 1881,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/hspec_case",
            "value": 219,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/naive_strip_each_read/heavy_ansi",
            "value": 5244,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_stripping_hotpath/cached_strip_once/heavy_ansi",
            "value": 548,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/single_file",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/small_batch",
            "value": 2794,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/medium_batch",
            "value": 80031,
            "range": "± 359",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/large_batch",
            "value": 1521435,
            "range": "± 9454",
            "unit": "ns/iter"
          },
          {
            "name": "file_event_processing/deep_hierarchy",
            "value": 131727,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_comparison",
            "value": 554387,
            "range": "± 12505",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_extension_check",
            "value": 30860,
            "range": "± 60",
            "unit": "ns/iter"
          },
          {
            "name": "path_operations/path_parent_extraction",
            "value": 43989,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/10",
            "value": 2387,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/10",
            "value": 2093,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/10",
            "value": 1981,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/100",
            "value": 58570,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/100",
            "value": 35264,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/100",
            "value": 68528,
            "range": "± 322",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/1000",
            "value": 978765,
            "range": "± 6345",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/1000",
            "value": 620855,
            "range": "± 4894",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/1000",
            "value": 1322166,
            "range": "± 5683",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/insert/5000",
            "value": 6140280,
            "range": "± 15286",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/contains/5000",
            "value": 3737811,
            "range": "± 18465",
            "unit": "ns/iter"
          },
          {
            "name": "btreeset_operations/merge/5000",
            "value": 8727545,
            "range": "± 24646",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/filter_haskell_files",
            "value": 97585,
            "range": "± 425",
            "unit": "ns/iter"
          },
          {
            "name": "event_filtering/deduplicate_events",
            "value": 593805,
            "range": "± 3230",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/ide_save_burst",
            "value": 25598,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/git_checkout",
            "value": 33655,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "file_events_real_world/build_artifacts_ignore",
            "value": 282170,
            "range": "± 3288",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/no_commands",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/few_commands",
            "value": 3486,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/many_commands",
            "value": 11021,
            "range": "± 140",
            "unit": "ns/iter"
          },
          {
            "name": "eval_command_detection/dense_commands",
            "value": 7660,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/small_file",
            "value": 9645,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/medium_file",
            "value": 142925,
            "range": "± 381",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/large_file",
            "value": 1609006,
            "range": "± 4525",
            "unit": "ns/iter"
          },
          {
            "name": "line_column_calc/very_long_lines",
            "value": 119650,
            "range": "± 451",
            "unit": "ns/iter"
          },
          {
            "name": "multiline_commands/parse_continuation_lines",
            "value": 11161,
            "range": "± 70",
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
            "value": 23997,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "eval_real_world/mixed_eval_styles",
            "value": 13321,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/no_ansi",
            "value": 53070,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/light_ansi",
            "value": 37442,
            "range": "± 1813",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/heavy_ansi",
            "value": 57593,
            "range": "± 552",
            "unit": "ns/iter"
          },
          {
            "name": "ansi_processing/mixed_content",
            "value": 50021,
            "range": "± 2213",
            "unit": "ns/iter"
          },
          {
            "name": "scrollback_buffer/append_line/small",
            "value": 3601,
            "range": "± 9",
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
            "value": 36062,
            "range": "± 144",
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
            "value": 319394,
            "range": "± 1426",
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
            "value": 12372,
            "range": "± 102",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/wrap_long_lines",
            "value": 3126,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "text_formatting/highlight_search",
            "value": 16431,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/progress_tracking",
            "value": 10321,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "compilation_status/format_durations",
            "value": 8450,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/full_compilation_render",
            "value": 53782,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/error_categorization",
            "value": 1974,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "tui_real_world/rapid_updates",
            "value": 2511,
            "range": "± 28",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}