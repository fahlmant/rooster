# Rooster

Rooster is a archiving tool written in Rust. The name comes from a kind of portmanteau
of `rust` and `ar`. 

Options
=======

Archive (-a)
------------

Archive file(s) and/or directory/directories specified

Usage: `rooster -a <archive_name>.roo <files_to_archive>`

The archive option takes at least two arguments. First, the name of the archive
file to be created, and then the files or directories to archive. This option
currently does not support appending to an existing archive

Extract (-e)
------------

Extract file(s) and/or directory/directories specified

Usage: `rooster -e <archive_name>.roo <files_to_extract>`

The extract option takes at least one agrument. The first argument is the 
archive file to extract from. If no files or directories are specified, all
contents of the archive will be extracted. Otherwise, only the specified files
or directories will be extracted.



