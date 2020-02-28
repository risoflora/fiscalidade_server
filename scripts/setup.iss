#define MyAppName ReadIni(AddBackslash(SourcePath) + "..\Cargo.toml", "package", "name")
#define MyAppVersion ReadIni(AddBackslash(SourcePath) + "..\Cargo.toml", "package", "version")
#define MyAppURL ReadIni(AddBackslash(SourcePath) + "..\Cargo.toml", "package", "repository")
#define MyAppDesc "Fiscalidade Server"
#define MyAppLongDesc "Servidor stand-alone com cache e APIs REST para envio e consulta de XMLs de Documentos Fiscais da SEFAZ."
#define MyAppArch "x64"
#define MyAppPublisher "Risoflora"

[Setup]
AppId={{8F7E9D0F-32B8-4B78-80B7-453F448CBCB2}
AppName={#MyAppDesc}
AppVersion={#MyAppVersion}
AppVerName={#MyAppDesc} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultGroupName={#MyAppDesc}
DefaultDirName={pf}\{#MyAppDesc}
LicenseFile=..\LICENSE-MIT
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename={#StringChange(MyAppDesc, ' ', '')}Setup-{#MyAppArch}-{#MyAppVersion}
Compression=lzma/ultra64
SolidCompression=yes
WizardStyle=modern
OutputDir=..\target\release
InternalCompressLevel=ultra64
SetupIconFile=..\resources\ico\{#MyAppName}.ico
UninstallDisplayIcon={app}\{#MyAppName}.ico

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
Source: "..\resources\ico\{#MyAppName}.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\LICENSE-MIT"; Filename: "{app}\LICENSE-MIT.txt"
Name: "{group}\LICENSE-APACHE"; Filename: "{app}\LICENSE-APACHE.txt"
Name: "{group}\{cm:UninstallProgram,{#MyAppDesc}}"; Filename: "{uninstallexe}"

[Run]
Filename: {sys}\sc.exe; Parameters: "create {#MyAppName} start= auto binPath= ""{app}\{#MyAppName}.exe"" DisplayName= ""{#MyAppDesc}"""; Flags: runhidden
Filename: {sys}\sc.exe; Parameters: "description {#MyAppName} ""{#MyAppLongDesc}"""; Flags: runhidden
Filename: {sys}\sc.exe; Parameters: "start {#MyAppName}"; Flags: runhidden

[UninstallRun]
Filename: {sys}\taskkill.exe; Parameters: "/f /im {#MyAppName}.exe"; Flags: skipifdoesntexist runhidden
;Filename: {sys}\sc.exe; Parameters: "stop {#MyAppName}"; Flags: runhidden
Filename: {sys}\sc.exe; Parameters: "delete {#MyAppName}"; Flags: runhidden

[Messages]
BeveledLabel= {#MyAppDesc} - {#MyAppURL}

[Code]
var
  DatabasePage: TInputQueryWizardPage;

function ConfigFileName: string;
begin
  Result := ExpandConstant('{pf}\{#MyAppDesc}\{#MyAppName}.conf');
end;

procedure InitializeWizard;
begin
  DatabasePage := CreateInputQueryPage(
    wpLicense,
    'Configurar banco de dados PostgreSQL',
    'É necessário informar uma URL de banco de dados PostgreSQL para prosseguir com a instalação',
    'A URL é composta de: <usuário>:<senha>@<servidor>[:porta]/<banco>');
  DatabasePage.Add('&URL da base de dados:', False);
  DatabasePage.Values[0] := ExpandConstant('postgres:postgres@localhost/postgres');
end;

function NextButtonClick(ACurPageID: Integer): Boolean;
begin
  Result := ACurPageID <> DatabasePage.ID;
  if Result then
    Exit;
  Result := DatabasePage.Values[0] <> '';
  if not Result then
    MsgBox('Você precisa informar a URL do banco de dados', mbError, MB_OK);
end;

function CreateConfigFile(const AFileName, ADatabaseURL: string): Boolean;
var
  VLines: TArrayOfString;
begin
  SetArrayLength(VLines, 3);
  VLines[0] := 'port=8080';
  VLines[1] := 'database=postgres://' + ADatabaseURL;
  VLines[2] := 'silent=true';
  Result := SaveStringsToFile(AFileName, VLines, True);
end;

procedure CurStepChanged(ACurStep: TSetupStep);
begin
  if ACurStep = ssPostInstall then
    CreateConfigFile(ConfigFileName, DatabasePage.Values[0]);
end;
