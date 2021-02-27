import usb.core
import usb.util

VID = 0x046d
PID = 0xc214

dev = usb.core.find(idVendor=VID, idProduct=PID)
if dev is None:
    raise ValueError('Device not found')

hadDriver = False
if dev.is_kernel_driver_active(0):
    dev.detach_kernel_driver(0)
    hadDriver = True

# set the active configuration. With no arguments, the first
# configuration will be the active one
dev.set_configuration()

# get an endpoint instance
cfg = dev.get_active_configuration()
intf = cfg[(0,0)]

ep = usb.util.find_descriptor(intf,
                    custom_match = \
                    lambda e: \
                        usb.util.endpoint_direction(e.bEndpointAddress) == \
                        usb.util.ENDPOINT_IN)

assert ep is not None

while True:
    try:
        ret = dev.read(ep, 100, 100)
        sret = ''.join(format(x, '02x') for x in ret)
        print(sret + '\t' + str(len(sret)))
        if (input("cmd: ") == "q"):
            break
    except:
        continue

usb.util.release_interface(dev, 0)
if hadDriver:
    dev.attach_kernel_driver(0)

# write the data
