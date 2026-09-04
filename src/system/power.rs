/*
todo: implement power profiles.


That gives you:

inactivity timer progresses to deeper sleep

ACTIVE ───────────────► IDLE ─────► LIGHT ─────► DEEP
▲                       ▲           ▲
│                       │           │
└──────────── activity ─┴───────────┘


                         ┌──────────────┐
                         │ POWER MANAGER│
                         └───────┬──────┘
                                 │
                         inactivity timer
                                 │
               ┌─────────────────┴─────────────────┐
               │                                   │
           no activity                         activity
               │                                   │
               ▼                                   ▼
          deeper sleep                         ACTIVE
               │
       ┌───────┴────────┐
       │                │
     timer             GPIO
       │                │
       └───────┬────────┘
               ▼
             WAKE



if your definition of "sleep" means:

"I still need my Wi-Fi connection to behave normally."

Your power manager needs to know about power constraints.

or example:

struct PowerPolicy {
wifi_connected: bool,
ble_connected: bool,
pending_work: bool,
last_activity: Instant,
}


Then:

Can sleep?
│
├── Wi-Fi needs to remain active? ──► No deep/light sleep
│
├── pending transaction? ───────────► Stay active
│
└── genuinely idle ─────────────────► Sleep


Then:

Can sleep?
│
├── Wi-Fi needs to remain active? ──► No deep/light sleep
│
├── pending transaction? ───────────► Stay active
│
└── genuinely idle ─────────────────► Sleep



   activity
     │
     ▼
┌─────────────┐
│   ACTIVE    │
│   100%      │
└──────┬──────┘
       │
idle for 100 ms
       ▼
┌─────────────┐
│   IDLE      │
│   100%      │
└──────┬──────┘
       │
idle for 1 sec
       ▼
┌─────────────┐
│ LIGHT SLEEP │
│   low       │
└──────┬──────┘
       │
idle for 30 sec
        ▼
┌─────────────┐
│ DEEP SLEEP  │
│  very low   │
└─────────────┘


And any activity kicks the system back toward ACTIVE:

sensor event ─────┐
button press ─────┤
radio event ──────┤
timer event ──────┤
                  ▼
                ACTIVE


That's a much better model than having every task independently decide when to sleep.

I'd make one task own the power state

For example:

enum PowerState {
Active,
Idle,
LightSleep,
DeepSleep,
}

prompt for extensions??
so deep slepp can be snozed in intervals of fibonacci series! wow cool


*/

log_info!();
