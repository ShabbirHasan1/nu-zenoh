#!/usr/bin/env nu
#
# Copy Zenoh-related commands from Nushell's history into Nuze's history.
#
# Run with:
#   nu scripts/migrate-history.nu

let old = (^nu -c '$nu.history-path' | str trim)
let new = (^nuze --print-history-path | str trim)

if not ($old | path exists) {
    print -e $"No Nushell history found at: ($old)"
    exit 0
}

mkdir ($new | path dirname)

let existing_text = if ($new | path exists) {
    open --raw $new
} else {
    ""
}

let existing = ($existing_text | lines)
let commands = (
    open --raw $old
    | lines
    | where {|cmd| $cmd == "zenoh" or ($cmd | str starts-with "zenoh ") }
    | where {|cmd| not ($cmd in $existing) }
)

if ($commands | is-empty) {
    print "No new Zenoh history entries to migrate"
    exit 0
}

if ($existing_text | is-not-empty) and not ($existing_text | str ends-with "\n") {
    "\n" | save --append $new
}

$commands | save --append $new

print $"Migrated ($commands | length) history entries from:\n  ($old)\nto:\n  ($new)"
