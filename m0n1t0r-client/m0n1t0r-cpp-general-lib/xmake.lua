includes("../xmake/ffi.lua")
add_rules("mode.debug", "mode.release")

add_requires("vcpkg::vmaware-vm-detection")

target("m0n1t0r-cpp-general-lib")
    set_kind("static")
    set_languages("c++20")
    on_load(function (target)
        target:add("ffi.rust.files", "../src/init.rs")
    end)
    set_rules("ffi.rust")

    add_packages("vcpkg::vmaware-vm-detection")

    add_includedirs("include")
    add_files("src/*.cpp")
