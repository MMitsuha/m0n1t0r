rule("ffi.rust")
    after_load(function (target)
        local file_paths = target:get("ffi.rust.files")
        for i, file_path in ipairs(file_paths) do
            print(i, ": Generating FFI for :", file_path)
            local file_name = path.filename(file_path)
            os.runv("cxxbridge", { file_path, "--header" }, { stdout = "include/" .. file_name .. ".h" })
            os.runv("cxxbridge", { file_path }, { stdout = "src/rs/" .. file_name .. ".cc" })
            target:add("files", "src/rs/" .. file_name .. ".cc")
            target:add("includedirs", "../../")
            target:add("includedirs", "../../target/cxxbridge/rust")
        end
    end)
