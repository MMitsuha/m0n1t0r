rule("config.dynamic_mode")
    before_build(function (target)
        if is_mode("release") then
            target:set("runtimes", "MD")
        else
            target:set("runtimes", "MDd")
        end
    end)

rule("config.static_mode")
    before_build(function (target)
        if is_mode("release") then
            target:set("runtimes", "MT")
        else
            target:set("runtimes", "MTd")
        end
    end)
