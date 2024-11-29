function add_dynamic_require(name, additional_config)
    full_name = name .. "~Dynamic"
    config = { configs = { shared = true } }

    if is_mode("release") then
        config.configs.runtimes = "MD"
        config.configs.debug = false
    else
        config.configs.runtimes = "MDd"
        config.configs.debug = true
    end

    if additional_config ~= nil then
        for key, value in pairs(additional_config) do
            config.configs[key] = value
        end
    end

    add_requires(full_name, config)
end

function add_static_require(name, additional_config)
    full_name = name .. "~Static"
    config = { configs = { shared = false } }

    if is_mode("release") then
        config.configs.runtimes = "MT"
        config.configs.debug = false
    else
        config.configs.runtimes = "MTd"
        config.configs.debug = true
    end

    if additional_config ~= nil then
        for key, value in pairs(additional_config) do
            config.configs[key] = value
        end
    end

    add_requires(full_name, config)
end

function add_full_require(name, additional_config)
    add_dynamic_require(name, additional_config)
    add_static_require(name, additional_config)
end

add_requires("nlohmann_json")
add_requires("spdlog")
add_requires("jwt-cpp")
add_requires("fmt")
add_requires("asio")
add_requires("xxhash")
add_requires("zlib")
add_requires("utfcpp")
add_requires("libcurl")
add_requires("openssl")
add_requires("cpr")
add_requires("boost", { configs = { coroutine = true } })
add_requires("magic_enum")
add_requires("openh264")
add_requires("m0n1t0r-sdk")

-- Used in server
add_requires("cli")

-- Used in test
add_requires("catch2")
