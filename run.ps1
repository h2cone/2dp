param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidateSet("start", "stop", "restart")]
    [string]$Command
)

$GodotExecutable = "godot"
$ProjectPath = "./godot"
$Arguments = @(
    "--path", $ProjectPath
)

$ProcessInfoFile = "godot_process.txt"

function Start-Godot {
    Write-Host "Attempting to start Godot project: $ProjectPath"
    Write-Host "Using Godot executable: $GodotExecutable"
    Write-Host "Arguments: $($Arguments -join ' ')"
    
    try {
        $godotProcess = Start-Process -FilePath $GodotExecutable -ArgumentList $Arguments -PassThru
        
        if ($godotProcess) {
            $processId = $godotProcess.Id
            Write-Host "Godot process started successfully."
            Write-Host "Process Name: $($godotProcess.ProcessName)"
            Write-Host "Process ID (PID): $processId"
            Write-Host "Command Line: $($godotProcess.CommandLine)"
            
            @{
                ProcessId = $processId
                StartTime = Get-Date
            } | ConvertTo-Json | Out-File -FilePath $ProcessInfoFile
            
            return $godotProcess
        }
        else {
            Write-Host "Failed to start the Godot process using Start-Process."
            Write-Host "Please check the Godot executable path and the project path."
            return $null
        }
    }
    catch {
        Write-Host "Error starting Godot: $_"
        return $null
    }
}

function Stop-Godot {
    if (Test-Path $ProcessInfoFile) {
        $processInfo = Get-Content $ProcessInfoFile | ConvertFrom-Json
        $process = Get-Process -Id $processInfo.ProcessId -ErrorAction SilentlyContinue
        
        if ($process -and -not $process.HasExited) {
            Write-Host "Stopping Godot process (PID: $($process.Id))..."
            $process.Kill()
            $process.WaitForExit()
            Write-Host "Godot process stopped."
            Remove-Item $ProcessInfoFile -ErrorAction SilentlyContinue
        }
        else {
            Write-Host "No running Godot process found."
            Remove-Item $ProcessInfoFile -ErrorAction SilentlyContinue
        }
    }
    else {
        Write-Host "No Godot process information found."
    }
}

function Restart-Godot {
    Stop-Godot
    Start-Sleep -Seconds 2
    
    Write-Host "Building Rust project..."
    try {
        $buildProcess = Start-Process -FilePath "cargo" -ArgumentList "build", "--manifest-path", "./rust/Cargo.toml" -Wait -NoNewWindow -PassThru
        
        Write-Host "Cargo build process completed with exit code: $($buildProcess.ExitCode)"
        
        if ($buildProcess.ExitCode -ne 0) {
            Write-Host "Rust build failed with exit code: $($buildProcess.ExitCode)"
            return $null
        }
        
        Write-Host "Rust build completed successfully. Starting Godot..."
        $godotProcess = Start-Godot
        if ($godotProcess) {
            Write-Host "Godot started successfully after restart"
        } else {
            Write-Host "Failed to start Godot after restart"
        }
        return $godotProcess
    }
    catch {
        Write-Host "Error during restart process: $_"
        return $null
    }
}

switch ($Command) {
    "start" {
        $godotProcess = Start-Godot
    }
    "stop" {
        Stop-Godot
    }
    "restart" {
        $godotProcess = Restart-Godot
    }
}