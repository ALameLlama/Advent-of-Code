local function compute_joltage(bank, required_batteries)
	local len = #bank
	local last_index = 1
	local joltage = ""

	for n = 1, required_batteries do
		local highest = { number = 0, index = 0 }

		-- use highest number index so we don't start from the beginning again
		-- use the required required_batteries - n loop to stop us checking all
		-- e.g if we have 10 in the bank and need 5
		-- first loop should be from 1 - 6 if we found the highest in there was 3rd place
		-- loop 2 should be 3 - 7 so we are only doing the smallest subset
		for i = last_index, len - (required_batteries - n) do
			local num = tonumber(bank:sub(i, i))
			if num > highest.number then
				highest.number = num
				highest.index = i
			end
		end

		joltage = joltage .. highest.number
		last_index = highest.index + 1
	end

	return tonumber(joltage)
end

-- OG part 1 before refactoring for part 2
-- local function part1()
-- 	local total = 0
--
-- 	-- for bank in io.lines("input.txt") do
-- 	-- 	local len = string.len(bank)
-- 	-- 	local highest1 = { number = 0, index = 0 }
-- 	-- 	local highest2 = { number = 0, index = 0 }
-- 	-- 	for x = 1, len - 1 do
-- 	-- 		local num = tonumber(bank:sub(x, x))
-- 	-- 		if num > highest1.number then
-- 	-- 			highest1.number = num
-- 	-- 			highest1.index = x
-- 	-- 		end
-- 	-- 	end
-- 	-- 	for y = highest1.index + 1, len do
-- 	-- 		local num = tonumber(bank:sub(y, y))
-- 	-- 		if num > highest2.number then
-- 	-- 			highest2.number = num
-- 	-- 			highest2.index = y
-- 	-- 		end
-- 	-- 	end
-- 	--
-- 	-- 	local joltage = tonumber(highest1.number .. highest2.number)
-- 	-- 	print(joltage)
-- 	-- 	total = total + joltage
-- 	-- end
-- 	--
-- 	print("Part 1: " .. total)
-- end

local function part1()
	local total = 0
	for bank in io.lines("input.txt") do
		total = total + compute_joltage(bank, 2)
	end
	print("Part 1: " .. total)
end

local function part2()
	local total = 0
	for bank in io.lines("input.txt") do
		total = total + compute_joltage(bank, 12)
	end
	print("Part 2: " .. string.format("%0.0f", total))
end

part1()
part2()
