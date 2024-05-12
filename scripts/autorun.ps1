# Add script argument
param(
    [string]$FileDirectory,
    [string]$FileName
)

$Prg = $FileDirectory + $FileName

function RegisterAutoRun
{
    # Name of the task
    $TaskName = "AutoRunRule"

    $UserName= $env:USERNAME

    # Set task action
    $TaskAction = New-ScheduledTaskAction -Execute $Prg

    # Create a trigger that will fire the task at log on
    $TaskTrigger = New-ScheduledTaskTrigger -AtLogOn -User $UserName

    # Register the task
    Register-ScheduledTask -Action $TaskAction -Trigger $TaskTrigger -User $UserName -RunLevel Highest -TaskName $TaskName -Force
}

RegisterAutoRun