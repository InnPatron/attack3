# NOTE: for some reason, I cannot do key injection and mouse injection at the same time
# Requires `sudo` to access dev/uinput

import time
from evdev import UInput, ecodes as e

uiKeyboard = UInput(events={
    e.EV_KEY: e.keys,
    })


#uiMouse = UInput(events={
#    e.EV_KEY: [e.BTN_LEFT],             # NOTE: necessary for some reason (at least for X11)
#    e.EV_REL: [e.REL_X, e.REL_Y],
#    })

# print(ui.capabilities(verbose=True))

print("Waiting...")
time.sleep(5)
print("Starting...")

#uiKeyboard.write(e.EV_KEY, e.KEY_A, 1)  # KEY_A down
#uiKeyboard.write(e.EV_KEY, e.KEY_A, 0)  # KEY_A up
#uiKeyboard.syn()

print("\nMouse...")

#for i in range(5):
#    uiMouse.write(e.EV_REL, e.REL_X, 10)
#    uiMouse.syn()
#    time.sleep(0.12);

time.sleep(2);
uiKeyboard.close()
# uiMouse.close()


print("Done!")
