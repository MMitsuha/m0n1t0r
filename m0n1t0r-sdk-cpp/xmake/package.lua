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

add_static_require("vcpkg::boost-random")
add_static_require("vcpkg::boost-filesystem")
add_static_require("vcpkg::boost-atomic")
add_static_require("websocketpp")
add_static_require("nlohmann_json")
add_static_require("spdlog")
add_static_require("fmt")
add_static_require("cpr")
add_static_require("upa-url")
add_static_require("cpp-channel")
