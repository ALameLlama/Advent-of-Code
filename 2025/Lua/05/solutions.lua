local function part1()
	local total = 0
	local ranges = {}

	local file, _ = io.open("input.txt", "r")
	local text = file:read("*all")
	local part1_txt, part2_txt = text:match("^(.-)\n%s*\n(.*)$")

	for range in (part1_txt .. "\n"):gmatch("(.-)\r?\n") do
		local x, y = range:match("(%d+)-(%d+)")

		table.insert(ranges, { start = tonumber(x), stop = tonumber(y) })
	end

	for ingrediant in (part2_txt .. "\n"):gmatch("(.-)\r?\n") do
		ingrediant = tonumber(ingrediant) or 0
		for _, range in pairs(ranges) do
			if ingrediant >= range.start and ingrediant <= range.stop then
				total = total + 1

				goto continue
			end
		end
		::continue::
	end

	print("Part 1: " .. total)
end

local function part2()
	local total = 0

	local file, _ = io.open("input.txt", "r")
	local text = file:read("*all")
	local part1_txt, _ = text:match("^(.-)\n%s*\n(.*)$")

	local ranges = {}

	for range in (part1_txt .. "\n"):gmatch("(.-)\r?\n") do
		local x, y = range:match("(%d+)-(%d+)")
		table.insert(ranges, { tonumber(x), tonumber(y) })
	end

	table.sort(ranges, function(a, b)
		return a[1] < b[1]
	end)

	local merged = { ranges[1] }

	for i = 2, #ranges do
		local last = merged[#merged]
		local curr = ranges[i]

		if curr[1] <= last[2] then
			last[2] = math.max(last[2], curr[2])
		else
			table.insert(merged, curr)
		end
	end

	for _, r in ipairs(merged) do
		total = total + (r[2] - r[1] + 1)
	end

	print("Part 2: " .. string.format("%d", total))
end

part1()
part2()
