iterate(list, action) := f"list='{{ list }}'; IFS='\n'; for each in $list; do {{ action }}; done"
