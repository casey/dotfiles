import XMonad
import XMonad.Actions.DwmPromote
import XMonad.Hooks.DynamicLog
import XMonad.Hooks.EwmhDesktops
import XMonad.Hooks.ManageDocks
import XMonad.Hooks.ManageHelpers
import XMonad.Layout.Gaps
import XMonad.Layout.Grid
import XMonad.Layout.MouseResizableTile
import XMonad.Layout.NoBorders
import XMonad.Layout.Spacing
import XMonad.Layout.Tabbed
import XMonad.Prompt
import XMonad.Prompt.Shell
import XMonad.Util.EZConfig
import XMonad.Util.Run
import XMonad.Util.SpawnOnce
import XMonad.Layout.Renamed
import Graphics.X11.ExtraTypes.XF86

main = xmonad $ docks $ ewmh $ def
    { modMask            = modm
    , borderWidth        = 1
    , focusedBorderColor = "#FF0000"
    , focusFollowsMouse  = False
    , handleEventHook    = eventHook' <+> handleEventHook def <+> fullscreenEventHook
    , layoutHook         = avoidStruts (tall ||| full)
    , normalBorderColor  = "#000000"
    , terminal           = "urxvt -e tmux"
    , manageHook         = manageHook' <+> manageHook def
    , workspaces         = ["一", "二", "三", "四", "五", "六", "七", "八", "九"]
    } `additionalKeys` keys
  where
    modm          = mod4Mask
    ctrlm         = controlMask
    modshiftm     = modm .|. shiftMask
-- gapspec       = [(U,10), (D,10), (R,10), (L,10)]
-- dragger       = FixedDragger 10 10
-- resizable     = smartBorders $ gaps gapspec $ mouseResizableTile { draggerType = dragger}
    tall          = renamed [Replace "tall" ] $ smartBorders $ spacingWithEdge 5 $ Tall 1 (3/100) (1/2)
    full          = renamed [Replace "full" ] $ smartBorders $ simpleTabbed
-- grid          = GridRatio (2560/1440)
    eventHook'   = mempty
    manageHook'  = composeAll
      [ className =? "Xmessage" --> doCenterFloat
      , className =? "XFontSel" --> doCenterFloat
      ]
    keys          =
      [ ((ctrlm,     xK_space               ), spawn "rofi -show run"                                     )
      , ((modm,      xK_BackSpace           ), spawn "sudo xautolock -locknow"                            )
      , ((modm,      xK_s                   ), spawn "scrot ~/dat/desktop/screen-%Y-%d-%m-%H-%M-%S.png"   )
      , ((modshiftm, xK_s                   ), spawn "scrot -u ~/dat/desktop/window-%Y-%d-%m-%H-%M-%S.png")
      , ((modm,      xK_o                   ), spawn "google-chrome-stable"                               )
      , ((modshiftm, xK_semicolon           ), shellPrompt def                                            )
      , ((modm,      xK_Return              ), dwmpromote                                                 )
      , ((0,         xF86XK_AudioLowerVolume), spawn "amixer set Master 5%-"                              )
      , ((0,         xF86XK_AudioRaiseVolume), spawn "amixer set Master 5%+"                              )
      , ((0,         xF86XK_AudioMute       ), spawn "amixer set Master toggle"                           )
      , ((0,         xF86XK_AudioNext       ), spawn "mpc next"                                           )
      , ((0,         xF86XK_AudioPrev       ), spawn "mpc prev"                                           )
      , ((0,         xF86XK_AudioStop       ), spawn "mpc stop"                                           )
      , ((0,         xF86XK_AudioPlay       ), spawn "mpc toggle"                                           )
      ]
