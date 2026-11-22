-- Pièce 1 pendant 10 trames à partir de la trame CPS_COIN, puis Start 1 pendant 10 trames
-- 120 trames plus tard (même séquence que l'exemple dump_audio de cps-core).
local c = tonumber(os.getenv("CPS_COIN") or "600")
local f = 0
local coin, start
for _, port in pairs(manager.machine.ioport.ports) do
  for name, field in pairs(port.fields) do
    if name == "Coin 1" then coin = field end
    if name == "1 Player Start" or name == "P1 Start" then start = field end
  end
end
emu.register_frame_done(function()
  f = f + 1
  local k = f + 1 -- valeur appliquée pendant la trame suivante
  if coin then coin:set_value((k >= c and k < c + 10) and 1 or 0) end
  if start then start:set_value((k >= c + 120 and k < c + 130) and 1 or 0) end
end)
