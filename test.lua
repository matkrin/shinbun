local M = {}

local channel_id

local function start_shinbun()
    channel_id = vim.fn.jobstart({ "cargo", "tauri", "dev", "--", "--", "--sync" })
    -- channel_id = vim.fn.jobstart({ "shinbun", "--sync" })
end

local function transfer_data()
    local buffer = vim.api.nvim_buf_get_lines(0, 0, -1, false)
    local joined = table.concat(buffer, "\n")
    local num_bytes_sent = vim.fn.chansend(channel_id, { joined, "" })
end

function M.setup()
        local group = vim.api.nvim_create_augroup("Shinbin", { clear = false })
        vim.api.nvim_create_autocmd({ "BufEnter" }, {
            group = group,
            pattern = { "*.md" },
            callback = function()
                start_shinbun()
                transfer_data()
            end
        })
        vim.api.nvim_create_autocmd({ "TextChanged", "TextChangedI", "TextChangedP" }, {
            group = group,
            pattern = { "*md" },
            callback = function()
                transfer_data()
            end
        })
end

return M
