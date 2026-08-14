; Tauri already uses MUI_FINISHPAGE_SHOWREADME for the desktop-shortcut checkbox
; and MUI_FINISHPAGE_RUN to launch the app. A finish-page link is the remaining
; MUI slot for opening the bundled README without replacing the whole template.
!define MUI_FINISHPAGE_LINK "README を開く"
!define MUI_FINISHPAGE_LINK_LOCATION "$INSTDIR\README.txt"
