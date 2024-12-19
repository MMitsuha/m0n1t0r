add_rules("mode.debug", "mode.release")

set_project("m0n1t0r-sdk")
set_version("0.0.1", {build = "%Y%m%d%H%M"})
set_license("LGPL-3.0")

includes("xmake")

target("m0n1t0r-sdk")
    set_kind("static")
    set_languages("cxx17")

    add_includedirs("include", { public = true })
    add_headerfiles("include/*.h")
    add_files("src/*.cpp")

    add_packages("vcpkg::boost-random~Static")
    add_packages("vcpkg::boost-filesystem~Static")
    add_packages("vcpkg::boost-atomic~Static")
    add_packages("vcpkg::websocketpp~Static")
    add_packages("fmt~Static")
    add_packages("spdlog~Static")
    add_packages("cpr~Static")
    add_packages("nlohmann_json~Static")
    add_packages("upa-url~Static")
    add_packages("cpp-channel~Static")
