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

add_dynamic_require("nlohmann_json")
add_dynamic_require("spdlog", {configs = {header_only = false}})
add_dynamic_require("jwt-cpp")
add_dynamic_require("fmt")
add_static_require("xxhash")
add_dynamic_require("zlib")
add_dynamic_require("utfcpp")
add_dynamic_require("libcurl")
add_dynamic_require("openssl")
add_dynamic_require("cpr")
add_dynamic_require("magic_enum")
add_dynamic_require("openh264")
add_static_require("m0n1t0r-sdk")
