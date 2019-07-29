import System.Taffybar

import System.Taffybar.Systray
import System.Taffybar.TaffyPager
import System.Taffybar.SimpleClock
import System.Taffybar.FreedesktopNotifications
import System.Taffybar.Weather
import System.Taffybar.MPRIS

import System.Taffybar.Widgets.PollingBar
import System.Taffybar.Widgets.PollingGraph

import System.Information.Memory
import System.Information.CPU

circle c = case c of
  '一' -> '㊀'
  '二' -> '㊁'
  '三' -> '㊂'
  '四' -> '㊃'
  '五' -> '㊄'
  '六' -> '㊅'
  '七' -> '㊆'
  '八' -> '㊇'
  '九' -> '㊈'
  '十' -> '㊉'
  x    -> x

parenthesize c = case c of
  '一' -> '㈠'
  '二' -> '㈡'
  '三' -> '㈢'
  '四' -> '㈣'
  '五' -> '㈤'
  '六' -> '㈥'
  '七' -> '㈦'
  '八' -> '㈧'
  '九' -> '㈨'
  '十' -> '㈩'
  x    -> x

color c s = "<span fgcolor='" ++ c ++ "'>" ++ s ++ "</span>"
orange = color "orange"
red    = color "red"

main = defaultTaffybar defaultTaffybarConfig
  { startWidgets = [pager]
  , endWidgets   = [tray, clock, mpris]
  }
  where
    mpris = mprisNew defaultMPRISConfig
    clock = textClockNew Nothing "%H%M" 1.0
    tray  = systrayNew
    pager = taffyPagerNew defaultPagerConfig
      { activeWindow     = const "🗔"
      , activeLayout     = id
      , activeWorkspace  = orange
      , hiddenWorkspace  = id
      , emptyWorkspace   = const ""
      , visibleWorkspace = const ""
      , urgentWorkspace  = red
      , widgetSep        = " "
      }
