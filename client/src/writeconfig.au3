#include <MsgBoxConstants.au3>
#include <File.au3>

Func WriteDefaultConfig($configFilePath, $webhookUrl)
    ; Create a constant variable in Local scope of the filepath that will be read/written to.
    Local Const $sTempFile = _TempFile()

    ; Write the section 'General'.
    IniWrite($sTempFile, "General", "WebHookUrl", $webhookUrl)

    ; Write the section 'AltNumberKeys'.
    IniWrite($sTempFile, "AltNumberKeys", "Alt1", "crickets")
    IniWrite($sTempFile, "AltNumberKeys", "Alt2", "easports")
    IniWrite($sTempFile, "AltNumberKeys", "Alt3", "i_believe_i_can_fly")
    IniWrite($sTempFile, "AltNumberKeys", "Alt4", "fart")
    IniWrite($sTempFile, "AltNumberKeys", "Alt5", "colombian_goal")
    IniWrite($sTempFile, "AltNumberKeys", "Alt6", "cl")
    IniWrite($sTempFile, "AltNumberKeys", "Alt7", "spass")
    IniWrite($sTempFile, "AltNumberKeys", "Alt8", "lets_get_ready_to_rumble")
    IniWrite($sTempFile, "AltNumberKeys", "Alt9", "nein")
    IniWrite($sTempFile, "AltNumberKeys", "Alt0", "over9000")

    ; Write the section 'CtrlNumberKeys'.
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl1", "crickets")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl2", "easports")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl3", "i_believe_i_can_fly")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl4", "fart")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl5", "colombian_goal")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl6", "cl")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl7", "spass")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl8", "lets_get_ready_to_rumble")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl9", "nein")
    IniWrite($sTempFile, "CtrlNumberKeys", "Ctrl0", "over9000")

    FileMove($sTempFile, $configFilePath, $FC_OVERWRITE)
EndFunc ;==>WriteDefaultConfig
