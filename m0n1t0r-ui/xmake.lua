add_rules("mode.debug", "mode.release")

set_project("m0n1t0r-ui")
set_version("0.0.1", {build = "%Y%m%d%H%M"})
set_license("LGPL-3.0")

includes("xmake")

target("s")
    set_kind("binary")
    set_languages("c++20")

    add_rules("config.dynamic_mode")
    add_rules("qt.widgetapp")

    add_includedirs("include")
    add_headerfiles("include/**/*.h")
    add_files("src/**/*.cpp")
    add_files("src/main.cpp")

    add_packages("spdlog~Dynamic")
    add_packages("magic_enum~Dynamic")
    add_packages("jwt-cpp~Dynamic")
    add_packages("nlohmann_json~Dynamic")
    add_packages("fmt~Dynamic")
    add_packages("cli~Dynamic")
    add_packages("boost~Dynamic")
    add_packages("openh264~Dynamic")

    -- add qt ui file
    add_files("src/**/*.ui")

    -- add files with Q_OBJECT meta (only for qt.moc)
    add_files("include/**/*.h")

    add_frameworks("QtNetwork", "QtWidgets", "QtGui", "QtCore", "QtWebSockets")


--
-- If you want to known more usage about xmake, please see https://xmake.io
--
-- ## FAQ
--
-- You can enter the project directory firstly before building project.
--
--   $ cd projectdir
--
-- 1. How to build project?
--
--   $ xmake
--
-- 2. How to configure project?
--
--   $ xmake f -p [macosx|linux|iphoneos ..] -a [x86_64|i386|arm64 ..] -m [debug|release]
--
-- 3. Where is the build output directory?
--
--   The default output directory is `./build` and you can configure the output directory.
--
--   $ xmake f -o outputdir
--   $ xmake
--
-- 4. How to run and debug target after building project?
--
--   $ xmake run [targetname]
--   $ xmake run -d [targetname]
--
-- 5. How to install target to the system directory or other output directory?
--
--   $ xmake install
--   $ xmake install -o installdir
--
-- 6. Add some frequently-used compilation flags in xmake.lua
--
-- @code
--    -- add debug and release modes
--    add_rules("mode.debug", "mode.release")
--
--    -- add macro definition
--    add_defines("NDEBUG", "_GNU_SOURCE=1")
--
--    -- set warning all as error
--    set_warnings("all", "error")
--
--    -- set language: c99, c++11
--    set_languages("c99", "c++11")
--
--    -- set optimization: none, faster, fastest, smallest
--    set_optimize("fastest")
--
--    -- add include search directories
--    add_includedirs("/usr/include", "/usr/local/include")
--
--    -- add link libraries and search directories
--    add_links("tbox")
--    add_linkdirs("/usr/local/lib", "/usr/lib")
--
--    -- add system link libraries
--    add_syslinks("z", "pthread")
--
--    -- add compilation and link flags
--    add_cxflags("-stdnolib", "-fno-strict-aliasing")
--    add_ldflags("-L/usr/local/lib", "-lpthread", {force = true})
--
-- @endcode
--
