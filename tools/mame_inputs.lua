-- Script MAME (0.287) : appuie sur des entrées à des trames précises, puis quitte.
--   CPS_PRESS = "900:COIN1:10,1000:START1:10"  (trame de début : type d'entrée MAME : durée en trames)
--   CPS_LAST  = "2400"                          (dernière trame ; MAME quitte ensuite)
-- Exemple (avec -wavwrite) : voir tools/README-golden.md.
local presses = {}
for f, t, d in string.gmatch(os.getenv("CPS_PRESS") or "", "(%d+):(%w+):(%d+)") do
  presses[#presses + 1] = { from = tonumber(f), to = tonumber(f) + tonumber(d), token = t }
end
local last = tonumber(os.getenv("CPS_LAST") or "1800")
local ioport = manager.machine.ioport
local function field_of(token)
  local wanted = ioport:token_to_input_type(token)
  for _, port in pairs(ioport.ports) do
    for _, field in pairs(port.fields) do
      if field.type == wanted then return field end
    end
  end
end
for _, p in ipairs(presses) do p.field = field_of(p.token) end
local count = 0
-- Garder la souscription dans une globale : sinon le ramasse-miettes Lua la supprime.
cps_inputs_subscription = emu.add_machine_frame_notifier(function()
  count = count + 1
  for _, p in ipairs(presses) do
    if p.field then p.field:set_value((count + 1 >= p.from and count + 1 < p.to) and 1 or 0) end
  end
  if count >= last then manager.machine:exit() end
end)
