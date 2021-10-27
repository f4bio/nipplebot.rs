; ====================================================
; ================= Example of a GUI =================
; ====================================================
; AutoIt version: 3.0.103
; Language:       English
; Author:         "SlimShady"
;
; ----------------------------------------------------------------------------
; Script Start
; ----------------------------------------------------------------------------

#include <GUIConstantsEx.au3>

Func ShowSettingsGui($firstTimeSetup, $configFilePath)
	;Initialize variables
	Local $iGUIWidth = 470, $iGUIHeight = 50
	Local $idWebhhokUrl_Label, $idWebhhokUrl_Input, $idWebhhokUrl_Button
	Local $resultData
	Local $windowTitle = $firstTimeSetup ? "Nipplecontrol: First Time Setup" : "Nipplecontrol: Settings"

	;Create window
	GUICreate($windowTitle, $iGUIWidth, $iGUIHeight)
	Opt("GUICoordMode", 2)
	
	$idWebhhokUrl_Label = GUICtrlCreateLabel("Webhook Url", 10, 10, 75)

	;Create an edit box with no text in it
	$idWebhhokUrl_Input = GUICtrlCreateInput("", 0, -1, 300)

	;Create an "OK" button
	$idOK_Btn = GUICtrlCreateButton("OK", 0, -1, 75)

	;Show window/Make the window visible
	GUISetState(@SW_SHOW)

	;Loop until:
	;- user presses Esc
	;- user presses Alt+F4
	;- user clicks the close button
	While 1
	    Switch GUIGetMsg()
			;Check if user clicked on the close button
			Case $GUI_EVENT_CLOSE
				;Destroy the GUI including the controls
				GUIDelete()
				;Exit the script
				Exit

				;Check if user clicked on the "OK" button
			Case $idOK_Btn
				$resultData = GUICtrlRead($idWebhhokUrl_Input)
				GUIDelete()
				Return $resultData

		EndSwitch
	WEnd
EndFunc   ;==>ShowSettingsGui
