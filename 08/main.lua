function hash_location(x, y, w, h)
  return (x - 1) + (w * (y - 1))
end

function main(input_file)
  local fp = io.open(input_file)
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
  local unique_locations = { }
  local antinode_count = 0

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
          unique_locations[hash_location(n, h, w, h)] = true
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

          while (ax > 0) and (ax <= w) and (ay > 0) and (ay <= h) do
            unique_locations[hash_location(ax, ay, w, h)] = true
            ax = ax + dx
            ay = ay + dy
          end
        end
      end
    end
  end

  -- print
  antinode_count = 0
  for k, _ in pairs(unique_locations) do
    antinode_count = antinode_count + 1
  end
  print(antinode_count)
end

main("sample.txt")
main("input.txt")

