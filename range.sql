SELECT
	*
FROM
	logs
WHERE
	id > (
		SELECT
			MAX(id) - 2000
		FROM
			logs
		WHERE
			isdir = 0
	)
;
