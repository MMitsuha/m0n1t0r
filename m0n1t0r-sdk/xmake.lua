add_rules("mode.debug", "mode.release")

set_project("m0n1t0r-sdk")
set_version("0.0.1", {build = "%Y%m%d%H%M"})
set_license("LGPL-3.0")

target("m0n1t0r-sdk")
    set_kind("binary")
    set_languages("c++20")

    add_includedirs("include")
    add_headerfiles("include/*.h")
    add_files("src/*.cpp")

    add_packages("vcpkg::websocketpp")
    add_packages("fmt")
    add_packages("spdlog")
    add_packages("cpr")
    add_packages("nlohmann_json")

includes("xmake")
