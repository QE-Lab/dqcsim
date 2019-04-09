#pragma once

/**
 * Extracts an array going by the name `marker` from a handle dump such that
 * modifications to the structure around this array don't affect this
 * function's output.
 */
inline char *extract_array_from_dump(const char *marker, char *dump) {
  int marker_len = strlen(marker);
  while (*dump) {
    if (!memcmp(dump, marker, marker_len)) {
      break;
    }
    dump++;
  }
  if (!*dump) return NULL;
  char *start = dump;
  char *out = dump;
  int level = 0;
  int after_newline = 0;
  while (*dump) {
    if (*dump == '[') {
      level++;
    } else if (*dump == ']') {
      level--;
      if (!level) {
        out[0] = ']';
        out[1] = 0;
        return start;
      }
    }
    if (after_newline) {
      if (*dump == ' ') {
        dump++;
        continue;
      } else {
        *out++ = ' ';
        after_newline = 0;
      }
    }
    if (*dump == '\n') {
      after_newline = 1;
      dump++;
      continue;
    }
    *out++ = *dump++;
  }
  return NULL;
}
