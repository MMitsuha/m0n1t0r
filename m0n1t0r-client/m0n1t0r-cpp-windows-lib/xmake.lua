includes("../xmake/ffi.lua")
add_rules("mode.debug", "mode.release")

add_requires("libpeconv")

target("m0n1t0r-cpp-windows-lib")
    set_kind("static")
    set_languages("c++20")
    on_load(function (target)
        target:add("ffi.rust.files", "../src/client/windows/process.rs")
        target:add("ffi.rust.files", "../src/client/windows/autorun.rs")
        target:add("ffi.rust.files", "../src/client/windows/charset.rs")
    end)
    set_rules("ffi.rust")

    add_packages("libpeconv")

    add_includedirs("include")
    add_files("src/*.cpp")
