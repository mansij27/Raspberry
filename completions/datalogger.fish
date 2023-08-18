complete -c datalogger -s p -l pin -d 'GPIO pin for DHT22 data connection' -r
complete -c datalogger -s i -l interval -d 'Interval between consecutive measures in seconds' -r
complete -c datalogger -s d -l directory -d 'Output CSV directory' -r -F
complete -c datalogger -s f -l format -d 'Output CSV filename format (see https://docs.rs/chrono/latest/chrono/format/strftime/index.html for valid specifiers)' -r
complete -c datalogger -s h -l help -d 'Print help information'
complete -c datalogger -s V -l version -d 'Print version information'
complete -c datalogger -s P -l pipe -d 'Print output as `<hum,temp>` to stdout (for use in unix pipeline)'
complete -c datalogger -l csv -d 'Dumps data to CSV file (can be swapped at runtime signalling `datalogger` process with SIGUSR1)'
complete -c datalogger -s q -l quiet -d 'Mute standard output'
