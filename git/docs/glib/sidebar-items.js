initSidebarItems({"attr":[["gflags","Attribute macro for defining flags using the `bitflags` crate. This macro will also define a `GFlags::type_` function and the `glib::Value` traits."],["object_interface","Macro for boilerplate of `ObjectInterface` implementations."],["object_subclass","Macro for boilerplate of `ObjectSubclass` implementations."]],"constant":[["CLONE_MACRO_LOG_DOMAIN","This is the log domain used by the [`clone!`][crate::clone!] macro. If you want to use a custom logger (it prints to stdout by default), you can set your own logger using the corresponding `log` functions."]],"derive":[["Downgrade","Macro for deriving implementations of `glib::clone::Downgrade` and `glib::clone::Upgrade` traits and a weak type."],["GBoxed","Derive macro for defining a `BoxedType``::type_` function and the `glib::Value` traits."],["GEnum",""],["GErrorDomain","Derive macro for defining a GLib error domain and its associated `ErrorDomain` trait."],["GSharedBoxed","Derive macro for defining a `SharedType``::get_type` function and the `glib::Value` traits."]],"enum":[["ChecksumType","The hashing algorithm to be used by [`Checksum`][crate::Checksum] when performing the digest of some data."],["DateMonth","Enumeration representing a month; values are [`January`][Self::January], [`February`][Self::February], etc. [`BadMonth`][Self::BadMonth] is the invalid value."],["DateWeekday","Enumeration representing a day of the week; [`Monday`][Self::Monday], [`Tuesday`][Self::Tuesday], etc. [`BadWeekday`][Self::BadWeekday] is an invalid weekday."],["FileError",""],["GlibLoggerDomain","Enumeration of the possible domain handling behaviours for a `GlibLogger`."],["GlibLoggerFormat","Enumeration of the possible formatting behaviours for a `GlibLogger`."],["KeyFileError","Error codes returned by key file parsing."],["LogLevel",""],["OptionArg","The [`OptionArg`][crate::OptionArg] enum values determine which type of extra argument the options expect to find. If an option expects an extra argument, it can be specified in several ways; with a short option: `-x arg`, with a long option: `--name arg` or combined in a single argument: `--name=arg`."],["SeekType","An enumeration specifying the base position for a `g_io_channel_seek_position()` operation."],["TimeType","Disambiguates a given time in two ways."],["UriError","Error codes returned by [`Uri`][crate::Uri] methods."],["UserDirectory","These are logical ids for special directories which are defined depending on the platform used. You should use [`user_special_dir()`][crate::user_special_dir()] to retrieve the full path associated to the logical id."]],"fn":[["access",""],["application_name",""],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["charset","Obtain the character set for the current locale."],["chdir",""],["check_version",""],["child_watch_future","Create a `Future` that will resolve once the child process with the given pid exits"],["child_watch_future_with_priority","Create a `Future` that will resolve once the child process with the given pid exits"],["clear_error",""],["codeset",""],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_checksum_for_string",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["compute_hmac_for_string",""],["console_charset",""],["current_dir",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ",""],["environ_getenv",""],["file_get_contents",""],["file_open_tmp",""],["file_read_link",""],["file_set_contents",""],["file_set_contents_full",""],["file_test",""],["filename_display_basename",""],["filename_display_name",""],["filename_from_uri",""],["filename_to_uri",""],["find_program_in_path",""],["format_size",""],["format_size_full",""],["getenv",""],["home_dir",""],["host_name",""],["hostname_is_ascii_encoded",""],["hostname_is_ip_address",""],["hostname_is_non_ascii",""],["hostname_to_ascii",""],["hostname_to_unicode",""],["interval_stream","Create a `Stream` that will provide a value every given number of milliseconds."],["interval_stream_seconds","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_seconds_with_priority","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_with_priority","Create a `Stream` that will provide a value every given number of milliseconds."],["is_canonical_pspec_name",""],["language_names",""],["language_names_with_category",""],["listenv",""],["locale_variants",""],["log_default_handler",""],["log_remove_handler",""],["log_set_always_fatal",""],["log_set_default_handler","To set back the default print handler, use the [`log_unset_default_handler`] function."],["log_set_fatal_mask",""],["log_set_handler",""],["log_unset_default_handler","To set the default print handler, use the [`log_set_default_handler`] function."],["main_current_source",""],["main_depth",""],["markup_escape_text",""],["mkdir_with_parents",""],["mkdtemp",""],["mkdtemp_full",""],["mkstemp",""],["mkstemp_full",""],["monotonic_time",""],["num_processors",""],["on_error_query",""],["on_error_stack_trace",""],["os_info",""],["path_get_basename",""],["path_get_dirname",""],["path_is_absolute",""],["path_skip_root",""],["pattern_match_simple",""],["prgname",""],["program_name","Same as `get_prgname()`."],["random_double","Returns a random `gdouble` equally distributed over the range [0..1)."],["random_double_range","Returns a random `gdouble` equally distributed over the range [`begin`..`end`)."],["random_int","Return a random `guint32` equally distributed over the range [0..2^32-1]."],["random_int_range","Returns a random `gint32` equally distributed over the range [`begin`..`end`-1]."],["random_set_seed","Sets the seed for the global random number generator, which is used by the g_random_* functions, to `seed`."],["real_name",""],["real_time",""],["reload_user_special_dirs_cache","Resets the cache used for [`user_special_dir()`][crate::user_special_dir()], so that the latest on-disk version is used. Call this only if you just changed the data on disk yourself."],["return_if_fail_warning","Internal function used to print messages from the public `g_return_if_fail()` and `g_return_val_if_fail()` macros."],["rmdir","A wrapper for the POSIX `rmdir()` function. The `rmdir()` function deletes a directory from the filesystem."],["rust_log_handler","Provides a glib log handler which routes all logging messages to the `log crate`."],["set_application_name","Sets a human-readable name for the application. This name should be localized if possible, and is intended for display to the user. Contrast with `g_set_prgname()`, which sets a non-localized name. `g_set_prgname()` will be called automatically by `gtk_init()`, but [`set_application_name()`][crate::set_application_name()] will not."],["set_prgname","Sets the name of the program. This name should not be localized, in contrast to [`set_application_name()`][crate::set_application_name()]."],["set_print_handler","To set back the default print handler, use the [`unset_print_handler`] function. Sets the print handler."],["set_printerr_handler","To set back the default print handler, use the [`unset_printerr_handler`] function. Sets the handler for printing error messages."],["set_program_name","Same as `set_prgname()`."],["setenv","Sets an environment variable. On UNIX, both the variable’s name and value can be arbitrary byte strings, except that the variable’s name cannot contain ‘=’. On Windows, they should be in UTF-8."],["shell_parse_argv","Parses a command line into an argument vector, in much the same way the shell would, but without many of the expansions the shell would perform (variable expansion, globs, operators, filename expansion, etc. are not supported). The results are defined to be the same as those you would get from a UNIX98 /bin/sh, as long as the input contains none of the unsupported shell expansions. If the input does contain such expansions, they are passed through literally. Possible errors are those from the `G_SHELL_ERROR` domain. Free the returned vector with `g_strfreev()`."],["shell_quote","Quotes a string so that the shell (/bin/sh) will interpret the quoted string to mean `unquoted_string`. If you pass a filename to the shell, for example, you should first quote it with this function. The return value must be freed with `g_free()`. The quoting style used is undefined (single or double quotes may be used)."],["shell_unquote","Unquotes a string as the shell (/bin/sh) would. Only handles quotes; if a string contains file globs, arithmetic operators, variables, backticks, redirections, or other special-to-the-shell features, the result will be different from the result a real shell would produce (the variables, backticks, etc. will be passed through literally instead of being expanded). This function is guaranteed to succeed if applied to the result of [`shell_quote()`][crate::shell_quote()]. If it fails, it returns [`None`] and sets the error. The `quoted_string` need not actually contain quoted or escaped text; [`shell_unquote()`][crate::shell_unquote()] simply goes through the string and unquotes/unescapes anything that the shell would. Both single and double quotes are handled, as are escapes including escaped newlines. The return value must be freed with `g_free()`. Possible errors are in the `G_SHELL_ERROR` domain."],["spaced_primes_closest","Gets the smallest prime number from a built-in array of primes which is larger than `num`. This is used within GLib to calculate the optimum size of a `GHashTable`."],["spawn_async","See `g_spawn_async_with_pipes()` for a full description; this function simply calls the `g_spawn_async_with_pipes()` without any pipes."],["spawn_async_with_fds","Identical to `g_spawn_async_with_pipes_and_fds()` but with `n_fds` set to zero, so no FD assignments are used."],["spawn_async_with_pipes","Identical to `g_spawn_async_with_pipes_and_fds()` but with `n_fds` set to zero, so no FD assignments are used."],["spawn_check_exit_status","Set `error` if `exit_status` indicates the child exited abnormally (e.g. with a nonzero exit code, or via a fatal signal)."],["spawn_command_line_async","A simple version of [`spawn_async()`][crate::spawn_async()] that parses a command line with [`shell_parse_argv()`][crate::shell_parse_argv()] and passes it to [`spawn_async()`][crate::spawn_async()]. Runs a command line in the background. Unlike [`spawn_async()`][crate::spawn_async()], the [`SpawnFlags::SEARCH_PATH`][crate::SpawnFlags::SEARCH_PATH] flag is enabled, other flags are not. Note that [`SpawnFlags::SEARCH_PATH`][crate::SpawnFlags::SEARCH_PATH] can have security implications, so consider using [`spawn_async()`][crate::spawn_async()] directly if appropriate. Possible errors are those from [`shell_parse_argv()`][crate::shell_parse_argv()] and [`spawn_async()`][crate::spawn_async()]."],["stpcpy","Copies a nul-terminated string into the dest buffer, include the trailing nul, and return a pointer to the trailing nul byte. This is useful for concatenating multiple strings together without having to repeatedly scan for the end."],["system_config_dirs",""],["system_data_dirs",""],["timeout_future","Create a `Future` that will resolve after the given number of milliseconds."],["timeout_future_seconds","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_seconds_with_priority","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_with_priority","Create a `Future` that will resolve after the given number of milliseconds."],["tmp_dir",""],["unix_open_pipe","Similar to the UNIX `pipe()` call, but on modern systems like Linux uses the `pipe2()` system call, which atomically creates a pipe with the configured flags. The only supported flag currently is `FD_CLOEXEC`. If for example you want to configure `O_NONBLOCK`, that must still be done separately with `fcntl()`."],["unix_signal_future","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_future_with_priority","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_stream","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unix_signal_stream_with_priority","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unlink","A wrapper for the POSIX `unlink()` function. The `unlink()` function deletes a name from the filesystem. If this was the last link to the file and no processes have it opened, the diskspace occupied by the file is freed."],["unset_print_handler","To set the default print handler, use the [`set_print_handler`] function."],["unset_printerr_handler","To set the default print handler, use the [`set_printerr_handler`] function."],["unsetenv","Removes an environment variable from the environment."],["user_cache_dir",""],["user_config_dir",""],["user_data_dir",""],["user_name",""],["user_runtime_dir",""],["user_special_dir",""],["usleep","Pauses the current thread for the given number of microseconds."],["uuid_string_is_valid","Parses the string `str` and verify if it is a UUID."],["uuid_string_random","Generates a random UUID (RFC 4122 version 4) as a string. It has the same randomness guarantees as `GRand`, so must not be used for cryptographic purposes such as key generation, nonces, salts or one-time pads."],["warn_message","Internal function used to print messages from the public `g_warn_if_reached()` and `g_warn_if_fail()` macros."]],"macro":[["bool_error","Generic error used for functions that fail without any further information"],["clone","Macro for passing variables as strong or weak references into a closure."],["debug","A macro which behaves exactly as `log::debug!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["error","A macro which behaves exactly as `log::error!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["g_critical","Macro used to log using GLib logging system. It uses g_log."],["g_debug","Macro used to log using GLib logging system. It uses g_log."],["g_error","Macro used to log using GLib logging system. It uses g_log."],["g_info","Macro used to log using GLib logging system. It uses g_log."],["g_log","Macro used to log using GLib logging system. It uses g_log."],["g_message","Macro used to log using GLib logging system. It uses g_log."],["g_print","Macro used to print messages. It uses g_print."],["g_printerr","Macro used to print error messages. It uses g_printerr."],["g_warning","Macro used to log using GLib logging system. It uses g_log."],["glib_boxed_wrapper","Wrapper implementations for Boxed types. See `wrapper!`."],["glib_object_wrapper","ObjectType implementations for Object types. See `wrapper!`."],["glib_shared_wrapper","Wrapper implementations for shared types. See `wrapper!`."],["info","A macro which behaves exactly as `log::info!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["result_from_gboolean",""],["trace","A macro which behaves exactly as `log::trace!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["warn","A macro which behaves exactly as `log::warn!` except that it sets the current log target to the contents of a `G_LOG_DOMAIN` constant (and fails to build if not defined)."],["wrapper","Defines a wrapper type and implements the appropriate traits."]],"mod":[["boxed","`IMPL` Boxed wrapper implementation."],["char",""],["clone",""],["closure",""],["error","`Error` binding and helper trait."],["functions",""],["object","`IMPL` Object wrapper implementation and `Object` binding."],["prelude","Traits and essential types intended for blanket imports."],["send_unique",""],["shared","`IMPL` Shared (reference counted) wrapper implementation."],["signal","`IMPL` Low level signal support."],["source",""],["subclass","Module containing infrastructure for subclassing `GObject`s and registering boxed types."],["translate","Translation between GLib/GLib-based FFI types and their Rust counterparts."],["types","Runtime type information."],["value","`Value` binding and helper traits."],["variant","`Variant` binding and helper traits."],["wrapper","`IMPL` The `wrapper!` macro and miscellaneous wrapper traits."]],"static":[["CSET_A_2_Z","The set of uppercase ASCII alphabet characters. Used for specifying valid identifier characters in `GScannerConfig`."],["CSET_DIGITS","The set of ASCII digits. Used for specifying valid identifier characters in `GScannerConfig`."],["CSET_a_2_z","The set of lowercase ASCII alphabet characters. Used for specifying valid identifier characters in `GScannerConfig`."],["KEY_FILE_DESKTOP_GROUP","The name of the main group of a desktop entry file, as defined in the Desktop Entry Specification. Consult the specification for more details about the meanings of the keys below."],["KEY_FILE_DESKTOP_KEY_ACTIONS","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string list giving the available application actions."],["KEY_FILE_DESKTOP_KEY_CATEGORIES","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings giving the categories in which the desktop entry should be shown in a menu."],["KEY_FILE_DESKTOP_KEY_COMMENT","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the tooltip for the desktop entry."],["KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean set to true if the application is D-Bus activatable."],["KEY_FILE_DESKTOP_KEY_EXEC","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the command line to execute. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_GENERIC_NAME","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the generic name of the desktop entry."],["KEY_FILE_DESKTOP_KEY_HIDDEN","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the desktop entry has been deleted by the user."],["KEY_FILE_DESKTOP_KEY_ICON","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the name of the icon to be displayed for the desktop entry."],["KEY_FILE_DESKTOP_KEY_MIME_TYPE","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings giving the MIME types supported by this desktop entry."],["KEY_FILE_DESKTOP_KEY_NAME","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a localized string giving the specific name of the desktop entry."],["KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings identifying the environments that should not display the desktop entry."],["KEY_FILE_DESKTOP_KEY_NO_DISPLAY","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the desktop entry should be shown in menus."],["KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a list of strings identifying the environments that should display the desktop entry."],["KEY_FILE_DESKTOP_KEY_PATH","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string containing the working directory to run the program in. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the application supports the Startup Notification Protocol Specification."],["KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is string identifying the WM class or name hint of a window that the application will create, which can be used to emulate Startup Notification with older applications."],["KEY_FILE_DESKTOP_KEY_TERMINAL","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a boolean stating whether the program should be run in a terminal window. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_TRY_EXEC","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the file name of a binary on disk used to determine if the program is actually installed. It is only valid for desktop entries with the `Application` type."],["KEY_FILE_DESKTOP_KEY_TYPE","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the type of the desktop entry. Usually [`KEY_FILE_DESKTOP_TYPE_APPLICATION`][crate::KEY_FILE_DESKTOP_TYPE_APPLICATION], [`KEY_FILE_DESKTOP_TYPE_LINK`][crate::KEY_FILE_DESKTOP_TYPE_LINK], or [`KEY_FILE_DESKTOP_TYPE_DIRECTORY`][crate::KEY_FILE_DESKTOP_TYPE_DIRECTORY]."],["KEY_FILE_DESKTOP_KEY_URL","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the URL to access. It is only valid for desktop entries with the `Link` type."],["KEY_FILE_DESKTOP_KEY_VERSION","A key under [`KEY_FILE_DESKTOP_GROUP`][crate::KEY_FILE_DESKTOP_GROUP], whose value is a string giving the version of the Desktop Entry Specification used for the desktop entry file."],["KEY_FILE_DESKTOP_TYPE_APPLICATION","The value of the [`KEY_FILE_DESKTOP_KEY_TYPE`][crate::KEY_FILE_DESKTOP_KEY_TYPE], key for desktop entries representing applications."],["KEY_FILE_DESKTOP_TYPE_DIRECTORY","The value of the [`KEY_FILE_DESKTOP_KEY_TYPE`][crate::KEY_FILE_DESKTOP_KEY_TYPE], key for desktop entries representing directories."],["KEY_FILE_DESKTOP_TYPE_LINK","The value of the [`KEY_FILE_DESKTOP_KEY_TYPE`][crate::KEY_FILE_DESKTOP_KEY_TYPE], key for desktop entries representing links to documents."],["OPTION_REMAINING","If a long option in the main group has this name, it is not treated as a regular option. Instead it collects all non-option arguments which would otherwise be left in `argv`. The option must be of type [`OptionArg::Callback`][crate::OptionArg::Callback], [`OptionArg::StringArray`][crate::OptionArg::StringArray] or [`OptionArg::FilenameArray`][crate::OptionArg::FilenameArray]."],["STR_DELIMITERS","The standard delimiters, used in `g_strdelimit()`."],["TEST_OPTION_ISOLATE_DIRS","Creates a unique temporary directory for each unit test and uses `g_set_user_dirs()` to set XDG directories to point into subdirectories of it for the duration of the unit test. The directory tree is cleaned up after the test finishes successfully. Note that this doesn’t take effect until `g_test_run()` is called, so calls to (for example) `g_get_user_home_dir()` will return the system-wide value when made in a test program’s `main()` function."],["URI_RESERVED_CHARS_GENERIC_DELIMITERS","Generic delimiters characters as defined in RFC 3986. Includes `:/?#[]@`."],["URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS","Subcomponent delimiter characters as defined in RFC 3986. Includes `!$&'()*+,;=`."],["g_param_spec_types",""]],"struct":[["Array",""],["Binding","[`Binding`][crate::Binding] is the representation of a binding between a property on a [`Object`][crate::Object] instance (or source) and another property on another [`Object`][crate::Object] instance (or target). Whenever the source property changes, the same value is applied to the target property; for instance, the following binding:"],["BindingFlags","Flags to be passed to [`ObjectExtManual::bind_property()`][crate::prelude::ObjectExtManual::bind_property()] or [`ObjectExtManual::bind_property_full()`][crate::prelude::ObjectExtManual::bind_property_full()]."],["ByteArray","Contains the public fields of a GByteArray."],["Bytes","A shared immutable byte slice (the equivalent of `Rc<[u8]>`)."],["Checksum","An opaque structure representing a checksumming operation. To create a new GChecksum, use [`new()`][Self::new()]. To free a GChecksum, use `g_checksum_free()`."],["Date",""],["DateTime","`GDateTime` is an opaque structure whose members cannot be accessed directly."],["EnumClass","Representation of an `enum` for dynamically, at runtime, querying the values of the enum and using them."],["EnumValue","Representation of a single enum value of an `EnumClass`."],["FileSetContentsFlags","Flags to pass to [`file_set_contents_full()`][crate::file_set_contents_full()] to affect its safety and performance."],["FileTest","A test to perform on a file using [`file_test()`][crate::file_test()]."],["FlagsBuilder","Builder for conveniently setting/unsetting flags and returning a `Value`."],["FlagsClass","Representation of a `flags` for dynamically, at runtime, querying the values of the enum and using them"],["FlagsValue","Representation of a single flags value of a `FlagsClass`."],["FormatSizeFlags","Flags to modify the format of the string returned by [`format_size_full()`][crate::format_size_full()]."],["GString",""],["GlibLogger","An implementation of a `log` compatible logger which logs over glib logging facilities."],["IOCondition","A bitwise combination representing a condition to watch for on an event source."],["KeyFile","The GKeyFile struct contains only private data and should not be accessed directly."],["KeyFileFlags","Flags which influence the parsing."],["LogHandlerId",""],["LogLevelFlags","Flags specifying the level of log messages."],["LogLevels",""],["MainContext","The `GMainContext` struct is an opaque data type representing a set of sources to be handled in a main loop."],["MainLoop","The `GMainLoop` struct is an opaque data type representing the main event loop of a GLib or GTK+ application."],["OptionFlags","Flags which modify individual options."],["ParamFlags",""],["ParamSpec",""],["ParamSpecBoolean",""],["ParamSpecBoxed",""],["ParamSpecChar",""],["ParamSpecDouble",""],["ParamSpecEnum",""],["ParamSpecFlags",""],["ParamSpecFloat",""],["ParamSpecGType",""],["ParamSpecInt",""],["ParamSpecInt64",""],["ParamSpecLong",""],["ParamSpecObject",""],["ParamSpecOverride",""],["ParamSpecParam",""],["ParamSpecPointer",""],["ParamSpecString",""],["ParamSpecUChar",""],["ParamSpecUInt",""],["ParamSpecUInt64",""],["ParamSpecULong",""],["ParamSpecUnichar",""],["ParamSpecValueArray",""],["ParamSpecVariant",""],["Quark",""],["Receiver","A `Receiver` that can be attached to a main context to receive items from its corresponding `Sender` or `SyncSender`."],["Sender","A `Sender` that can be used to send items to the corresponding main context receiver."],["SignalFlags",""],["Source","The `GSource` struct is an opaque data type representing an event source."],["SourceFuture","Represents a `Future` around a `glib::Source`. The future will be resolved once the source has provided a value"],["SourceStream","Represents a `Stream` around a `glib::Source`. The stream will be provide all values that are provided by the source"],["SpawnFlags","Flags passed to [`spawn_sync()`][crate::spawn_sync()], [`spawn_async()`][crate::spawn_async()] and `g_spawn_async_with_pipes()`."],["String","A mutable text buffer that grows automatically."],["SyncSender","A `SyncSender` that can be used to send items to the corresponding main context receiver."],["ThreadPool",""],["TimeZone","[`TimeZone`][crate::TimeZone] is an opaque structure whose members cannot be accessed directly."],["Uri","The [`Uri`][crate::Uri] type and related functions can be used to parse URIs into their components, and build valid URIs from individual components."],["UriFlags","Flags that describe a URI."],["UriHideFlags","Flags describing what parts of the URI to hide in [`Uri::to_string_partial()`][crate::Uri::to_string_partial()]. Note that [`PASSWORD`][Self::PASSWORD] and [`AUTH_PARAMS`][Self::AUTH_PARAMS] will only work if the [`Uri`][crate::Uri] was parsed with the corresponding flags."],["UriParamsFlags","Flags modifying the way parameters are handled by [`Uri::parse_params()`][crate::Uri::parse_params()] and `GUriParamsIter`."],["ValueArray",""],["VariantDict","`VariantDict` is a mutable key/value store where the keys are always strings and the values are `Variant`s."],["VariantIter","Iterator over items in a variant."],["VariantTy","Describes `Variant` types."],["VariantType","Describes `Variant` types."]],"trait":[["ParamSpecType",""]],"type":[["DateDay",""],["DateYear",""],["Time",""],["TimeSpan",""]]});