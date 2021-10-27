#include <MsgBoxConstants.au3>
#include "./HTTP.au3"
#include "./JSON.au3"
#include "./settingsgui.au3"
#include "./writeconfig.au3"

Global $DEBUG_MODE = EnvGet("DEBUG_MODE")
Global $g_bPaused = False
Global $configFilePath = "nippleconfig.ini"

Local $iFileExists = FileExists($configFilePath)
Local $currentSettings

Local $webHookUrl
Local $alt1, $alt2, $alt3, $alt4, $alt5, $alt6, $alt7, $alt8, $alt9, $alt0
Local $ctrl1, $ctrl2, $ctrl3, $ctrl4, $ctrl5, $ctrl6, $ctrl7, $ctrl8, $ctrl9, $ctrl0

If $DEBUG_MODE Or Not $iFileExists Then
   $currentSettings = ShowSettingsGui(True)
   WriteDefaultConfig($configFilePath, $currentSettings)
EndIf

; Read the INI file for the value of 'Title' in the section labelled 'General'.
$webHookUrl = IniRead($configFilePath, "General", "WebHookUrl", "https://discordapp.com/api/webhooks/1337133713371337/abc-def-_ghi")
; alt1 - alt0
$alt1 = IniRead($configFilePath, "AltNumberKeys", "Alt1", "crickets")
$alt2 = IniRead($configFilePath, "AltNumberKeys", "Alt2", "easports")
$alt3 = IniRead($configFilePath, "AltNumberKeys", "Alt3", "i_believe_i_can_fly")
$alt4 = IniRead($configFilePath, "AltNumberKeys", "Alt4", "fart")
$alt5 = IniRead($configFilePath, "AltNumberKeys", "Alt5", "fuck")
$alt6 = IniRead($configFilePath, "AltNumberKeys", "Alt6", "cl")
$alt7 = IniRead($configFilePath, "AltNumberKeys", "Alt7", "nein")
$alt8 = IniRead($configFilePath, "AltNumberKeys", "Alt8", "lets_get_ready_to_rumble")
$alt9 = IniRead($configFilePath, "AltNumberKeys", "Alt9", "trombone")
$alt0 = IniRead($configFilePath, "AltNumberKeys", "Alt0", "over9000")
; ctrl1 - ctrl0
$ctrl1 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl1", "colombian_goal")
$ctrl2 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl2", "hot_hot_hot")
$ctrl3 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl3", "sad_violins")
$ctrl4 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl4", "tor1")
$ctrl5 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl5", "tor2")
$ctrl6 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl6", "verantwortung")
$ctrl7 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl7", "wtf_is_going_on")
$ctrl8 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl8", "lets_go")
$ctrl9 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl9", "wtf")
$ctrl0 = IniRead($configFilePath, "CtrlNumberKeys", "Ctrl0", "tadaah")

HotKeySet("!1", "HotKeyPressed") ; Alt-1
HotKeySet("!2", "HotKeyPressed") ; Alt-2
HotKeySet("!3", "HotKeyPressed") ; Alt-3
HotKeySet("!4", "HotKeyPressed") ; Alt-4
HotKeySet("!5", "HotKeyPressed") ; Alt-5
HotKeySet("!6", "HotKeyPressed") ; Alt-6
HotKeySet("!7", "HotKeyPressed") ; Alt-7
HotKeySet("!8", "HotKeyPressed") ; Alt-8
HotKeySet("!9", "HotKeyPressed") ; Alt-9
HotKeySet("!0", "HotKeyPressed") ; Alt-0
HotKeySet("^1", "HotKeyPressed") ; Ctrl-1
HotKeySet("^2", "HotKeyPressed") ; Ctrl-2
HotKeySet("^3", "HotKeyPressed") ; Ctrl-3
HotKeySet("^4", "HotKeyPressed") ; Ctrl-4
HotKeySet("^5", "HotKeyPressed") ; Ctrl-5
HotKeySet("^6", "HotKeyPressed") ; Ctrl-6
HotKeySet("^7", "HotKeyPressed") ; Ctrl-7
HotKeySet("^8", "HotKeyPressed") ; Ctrl-8
HotKeySet("^9", "HotKeyPressed") ; Ctrl-9
HotKeySet("^0", "HotKeyPressed") ; Ctrl-0

While 1
  Sleep(100)
WEnd

Func HotKeyPressed()
  Switch @HotKeyPressed ; The last hotkey pressed.
   Case "!1" ; String is the Alt-1 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt1)))
   Case "!2" ; String is the Alt-2 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt2)))
   Case "!3" ; String is the Alt-3 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt3)))
   Case "!4" ; String is the Alt-4 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt4)))
   Case "!5" ; String is the Alt-5 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt5)))
   Case "!6" ; String is the Alt-6 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt6)))
   Case "!7" ; String is the Alt-7 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt7)))
   Case "!8" ; String is the Alt-8 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt8)))
   Case "!9" ; String is the Alt-9 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt9)))
   Case "!0" ; String is the Alt-0 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $alt0)))
   Case "^1" ; String is the Ctrl-1 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl1)))
   Case "^2" ; String is the Ctrl-2 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl2)))
   Case "^3" ; String is the Ctrl-3 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl3)))
   Case "^4" ; String is the Ctrl-4 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl4)))
   Case "^5" ; String is the Ctrl-5 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl5)))
   Case "^6" ; String is the Ctrl-6 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl6)))
   Case "^7" ; String is the Ctrl-7 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl7)))
   Case "^8" ; String is the Ctrl-8 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl8)))
   Case "^9" ; String is the Ctrl-9 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl9)))
   Case "^0" ; String is the Ctrl-0 hotkey
      _HTTP_Post($webHookUrl, _JSONEncode(_JSONObject("content", "!" & $ctrl0)))
  EndSwitch
EndFunc   ;==>HotKeyPressed
