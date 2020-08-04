SELECT	ip
FROM	logs
WHERE	id > (	SELECT MAX(id) - 2000
		FROM logs
		WHERE isdir = 0 )
	AND cmd = 'delete'
	AND time > (	SELECT MAX(time)
			FROM logs ) - 100
;
