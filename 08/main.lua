function main()
  local fp = io.open("input.txt")
  local lines = fp:lines()
  local first_line = true
  local antenna_groups = { }
  local w = 0
  local h = 0
  local c = "c"
  local antenna_group = { }
  local x = 0
  local y = 0
  local x1 = 0
  local y1 = 0
  local dx = 0
  local dy = 0
  local ax = 0  -- antinode x
  local ay = 0  -- antinode y
  local ah = 0  -- antinode position hash
  local unique_locations = { }
  local result = 0

  -- read
  for line in lines do
    if #line > 0 then
      if first_line == true then
        w = #line
        first_line = false
      end
      h = h + 1

      for n = 1, w do
        c = line:sub(n, n)
        if c ~= "." then
          antenna_group = antenna_groups[c]
          if antenna_group == nil then
            antenna_group = { }
          end
          table.insert(antenna_group, {n, h})
          antenna_groups[c] = antenna_group
        end
      end
    end
  end

  -- evaluate
  for frequency, group in pairs(antenna_groups) do
    for i, reference_antenna in pairs(group) do
      x = reference_antenna[1]
      y = reference_antenna[2]

      for j, antenna in pairs(group) do
        if i ~= j then
          x1 = antenna[1]
          y1 = antenna[2]
          dx = x1 - x
          dy = y1 - y
          ax = x1 + dx
          ay = y1 + dy

          if (ax > 0) and (ax <= w) and (ay > 0) and (ay <= h) then
            ah = (ax - 1) + (w * (ay - 1))
            unique_locations[ah] = true
          end
        end
      end
    end
  end

  -- print
  for k, _ in pairs(unique_locations) do
    result = result + 1
  end
  print(result)
end

main()

