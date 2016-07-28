# Rooster

Rooster is a archiving tool written in Rust. The name comes from a kind of portmanteau
of `rust` and `ar`. 

Options
=======

Archive (-a)
------------

Archive file(s) and/or directory/directories specified to archive specified.

Usage: `rooster -a <archive_name>.roo <files_to_archive>`

The archive option takes at least two arguments. First, the name of the archive
file to be created, and then the files or directories to archive. This option
currently does not support appending to an existing archive

Extract (-e)
------------

Extract file(s) and/or directory/directories specified from archive specified.

Usage: `rooster -e <archive_name>.roo <files_to_extract>`

The extract option takes at least one agrument. The first argument is the 
archive file to extract from. If no files or directories are specified, all
contents of the archive will be extracted. Otherwise, only the specified files
or directories will be extracted.

Delete (-d)
-----------

Delete file(s) and/or directory/directories specified from archive specificed.

Usage: `rooster -d <archive_name>.roo <files_to_delete>`

The delete option takes at least two arguments. The first argument is the 
archive file to delete from. The second and further arguments are the items to
delete from the archive. At least one item must be selected and exist in the
archive. If a directory is selected, all files under that directory will be 
removed. Precedent of deletion is from first argument to last, so if a file is
specified but has already been deleted from the archive, the program will exit.

NOTE: If two files exist in the archive with the same name, as of the current
version (0.1.0), only the first occurance will be deleted. 

Overwrite (-o)
--------------

The overwrite option enables the overwritting of files from an extract.

Usage: `rooster -eo <archive_name>.roo <files_to_extract>`

Overwrite must be used with extract. When extracting all specified items, if 
the item already exists in the present working directory, the overwrite option
will force them to be overwritten by the extract. 


Help (-h)
---------

Displays the help message (same as the README).

Usage: `rooster -h`


Short Table of Contents (-t)
----------------------------

Displays the table of contents of an archive. 

Usage: `rooster -t <archive_name>.roo`

The short table of contents shows the drictories and files in an archive.

Long Table of Contents (-T)
---------------------------

Displays the long table of contents of an archive.

Usage: `rooster -T <archive_name>.roo`

Version (-V)
------------

Displays the version of Rooster being used.

Usage: `rooster -V`

Verbose Mode (-v)
-----------------

Increases verbosity of output for Archive, Extract, Delete, Overwrite and 
Tables. 

Usage `rooster -(a,e,d,o,t,T) -v <arguments>`


Contributing
============
