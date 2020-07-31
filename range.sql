SELECT
	*
FROM
	logs
WHERE
	id > (
		SELECT MAX(id) - $range
		FROM logs
	)
;
