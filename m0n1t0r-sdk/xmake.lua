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

    add_packages("websocketpp")
    add_packages("fmt")
    add_packages("spdlog")
    add_packages("cpr")
    add_packages("nlohmann_json")
    add_packages("boost")
