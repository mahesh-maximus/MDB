set fish_color_command yellow --bold
set fish_color_autosuggestion white

function fish_prompt -d "Write out the prompt"
	# printf '%s@%s %s%s%s ->' $USER $hostname (set_color $fish_color_cwd) (prompt_pwd) (set_color normal)
	printf '%s %s%s%s ->' $USER (set_color $fish_color_cwd) (prompt_pwd) (set_color normal)
end

function fish_greeting
	set_color red --bold
     	echo MAHESH WORLD
	set_color normal

	echo
	echo -e (uname -ro | awk '{print " \\\\e[1mOS: \\\\e[0;32m"$0"\\\\e[0m"}')
	echo -e (uptime -p | sed 's/^up //' | awk '{print " \\\\e[1mUptime: \\\\e[0;32m"$0"\\\\e[0m"}')
	echo -e (uname -n | awk '{print " \\\\e[1mHostname: \\\\e[0;32m"$0"\\\\e[0m"}')

	echo
     	set_color normal
end
