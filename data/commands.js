[
	// temp:no halve clone png:100 640x480
	//
	// FLAG - standalone argument, does not have any parameters
	// COLON - accepts one or more paramerets (params)
	// MATCH - parameter is defined by a regular expression with arguments being capture groups
	{
		name: "temp",
		description: "Action group related to backup",
		structure: {
			type: "COLON",
			values: {
				no: {
					description: "Do not backup",
                    // type: "string" // implicitly set to be 'string'
				},
				yes: {
					// use later?
					description: "Do backup (default)",
				},
				clear: {
					description: "Remove all backups defined in foto.bk",
					values: {
						// looks useless, but i have to leave it here
						// as an example of double parameter colon flag:
						// temp:clear:2
						"[0-9]+": {
							description: "Remove nth backup from foto.bk",
						},
					},
				},
                save: { // expose this because why not?
                    description: "Backup files (implicitly called before exiting when backup:yes)"
                }
			},
		},
	},
	{
		name: "halve",
		description: "Halve the resolution",
		structure: {
			type: "FLAG",
		},
	},
    {
        name: "(png|jpg):([1-9][0-9]?0?)",
        description: "Set type:quality of the output file (0-100, higher is better)",
        structure: {
            type: "MATCH",
            types: ["u8"],
        }
    }
];
