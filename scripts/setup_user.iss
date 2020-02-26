#define MyAppName ReadIni(AddBackslash(SourcePath) + "..\Cargo.toml", "package", "name")
#define MyAppVersion ReadIni(AddBackslash(SourcePath) + "..\Cargo.toml", "package", "version")
#define MyAppURL ReadIni(AddBackslash(SourcePath) + "..\Cargo.toml", "package", "repository")
#define MyAppDesc "Fiscalidade Server"
#define MyAppArch "x64"
#define MyAppPublisher "Risoflora"

[Setup]
AppId={{B2B339F1-5EE4-4A4A-A90E-F56EB3EDA1E4}
AppName={#MyAppDesc}
AppVersion={#MyAppVersion}
AppVerName={#MyAppDesc} {#MyAppVersion} (User)
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultGroupName={#MyAppDesc}
DefaultDirName={autopf}\{#MyAppDesc}
LicenseFile=..\LICENSE-MIT
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequired=lowest
OutputBaseFilename={#MyAppName}-{#MyAppArch}-{#MyAppVersion}
Compression=lzma/ultra64
SolidCompression=yes
WizardStyle=modern
OutputDir=..\target\release
InternalCompressLevel=ultra64

[Languages]
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[Files]
Source: "..\target\release\{#MyAppName}.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\libcrypto-1_1-x64.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\libiconv-2.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\libintl-8.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\libpq.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\libssl-1_1-x64.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE-APACHE"; DestDir: "{app}"; DestName: "LICENSE-APACHE.txt"; Flags: ignoreversion
Source: "..\LICENSE-MIT"; DestDir: "{app}"; DestName: "LICENSE-MIT.txt"; Flags: ignoreversion

[Icons]
Name: "{group}\LICENSE-MIT"; Filename: "{app}\LICENSE-MIT.txt"
Name: "{group}\LICENSE-APACHE"; Filename: "{app}\LICENSE-APACHE.txt"
Name: "{group}\{cm:UninstallProgram,{#MyAppDesc}}"; Filename: "{uninstallexe}"
Name: "{userstartup}\{#MyAppDesc}"; Filename: "{app}\{#MyAppName}.exe"; WorkingDir: "{app}"

[Messages]
BeveledLabel= {#MyAppDesc} - {#MyAppURL}

[Run]
Filename: "{app}\{#MyAppName}.exe"; Description: "{cm:LaunchProgram,{#StringChange(MyAppDesc, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: {sys}\taskkill.exe; Parameters: "/f /im {#MyAppName}.exe"; Flags: skipifdoesntexist runhidden
