DECLARE @MULTIPLEXER AS NVARCHAR(255) = ''
DECLARE @RESULT AS NVARCHAR(MAX) = N''

DECLARE @D0 AS NVARCHAR(255) = '0'
DECLARE @D1 AS NVARCHAR(255) = '1'
DECLARE @D2 AS NVARCHAR(255) = '2'
DECLARE @D3 AS NVARCHAR(255) = '3'
DECLARE @D4 AS NVARCHAR(255) = '4'
DECLARE @D5 AS NVARCHAR(255) = '5'
DECLARE @D6 AS NVARCHAR(255) = '6'
DECLARE @D7 AS NVARCHAR(255) = '7'
DECLARE @D8 AS NVARCHAR(255) = '8'
DECLARE @D9 AS NVARCHAR(255) = '9'

WHILE (1 = 1)
BEGIN
	
	DECLARE @DS0 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D0)), 1) + '00000000000000000000000000000000', 42)
	SET @DS0 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS0, 41, 2) + SUBSTRING(@DS0, 39, 2) + SUBSTRING(@DS0, 37, 2) + SUBSTRING(@DS0, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS0, 33, 2) + SUBSTRING(@DS0, 31, 2) + SUBSTRING(@DS0, 29, 2) + SUBSTRING(@DS0, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS0, 25, 2) + SUBSTRING(@DS0, 23, 2) + SUBSTRING(@DS0, 21, 2) + SUBSTRING(@DS0, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS0, 17, 2) + SUBSTRING(@DS0, 15, 2) + SUBSTRING(@DS0, 13, 2) + SUBSTRING(@DS0, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS1 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D1)), 1) + '00000000000000000000000000000000', 42)
	SET @DS1 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS1, 41, 2) + SUBSTRING(@DS1, 39, 2) + SUBSTRING(@DS1, 37, 2) + SUBSTRING(@DS1, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS1, 33, 2) + SUBSTRING(@DS1, 31, 2) + SUBSTRING(@DS1, 29, 2) + SUBSTRING(@DS1, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS1, 25, 2) + SUBSTRING(@DS1, 23, 2) + SUBSTRING(@DS1, 21, 2) + SUBSTRING(@DS1, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS1, 17, 2) + SUBSTRING(@DS1, 15, 2) + SUBSTRING(@DS1, 13, 2) + SUBSTRING(@DS1, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS2 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D2)), 1) + '00000000000000000000000000000000', 42)
	SET @DS2 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS2, 41, 2) + SUBSTRING(@DS2, 39, 2) + SUBSTRING(@DS2, 37, 2) + SUBSTRING(@DS2, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS2, 33, 2) + SUBSTRING(@DS2, 31, 2) + SUBSTRING(@DS2, 29, 2) + SUBSTRING(@DS2, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS2, 25, 2) + SUBSTRING(@DS2, 23, 2) + SUBSTRING(@DS2, 21, 2) + SUBSTRING(@DS2, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS2, 17, 2) + SUBSTRING(@DS2, 15, 2) + SUBSTRING(@DS2, 13, 2) + SUBSTRING(@DS2, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS3 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D3)), 1) + '00000000000000000000000000000000', 42)
	SET @DS3 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS3, 41, 2) + SUBSTRING(@DS3, 39, 2) + SUBSTRING(@DS3, 37, 2) + SUBSTRING(@DS3, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS3, 33, 2) + SUBSTRING(@DS3, 31, 2) + SUBSTRING(@DS3, 29, 2) + SUBSTRING(@DS3, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS3, 25, 2) + SUBSTRING(@DS3, 23, 2) + SUBSTRING(@DS3, 21, 2) + SUBSTRING(@DS3, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS3, 17, 2) + SUBSTRING(@DS3, 15, 2) + SUBSTRING(@DS3, 13, 2) + SUBSTRING(@DS3, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS4 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D4)), 1) + '00000000000000000000000000000000', 42)
	SET @DS4 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS4, 41, 2) + SUBSTRING(@DS4, 39, 2) + SUBSTRING(@DS4, 37, 2) + SUBSTRING(@DS4, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS4, 33, 2) + SUBSTRING(@DS4, 31, 2) + SUBSTRING(@DS4, 29, 2) + SUBSTRING(@DS4, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS4, 25, 2) + SUBSTRING(@DS4, 23, 2) + SUBSTRING(@DS4, 21, 2) + SUBSTRING(@DS4, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS4, 17, 2) + SUBSTRING(@DS4, 15, 2) + SUBSTRING(@DS4, 13, 2) + SUBSTRING(@DS4, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS5 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D5)), 1) + '00000000000000000000000000000000', 42)
	SET @DS5 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS5, 41, 2) + SUBSTRING(@DS5, 39, 2) + SUBSTRING(@DS5, 37, 2) + SUBSTRING(@DS5, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS5, 33, 2) + SUBSTRING(@DS5, 31, 2) + SUBSTRING(@DS5, 29, 2) + SUBSTRING(@DS5, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS5, 25, 2) + SUBSTRING(@DS5, 23, 2) + SUBSTRING(@DS5, 21, 2) + SUBSTRING(@DS5, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS5, 17, 2) + SUBSTRING(@DS5, 15, 2) + SUBSTRING(@DS5, 13, 2) + SUBSTRING(@DS5, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS6 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D6)), 1) + '00000000000000000000000000000000', 42)
	SET @DS6 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS6, 41, 2) + SUBSTRING(@DS6, 39, 2) + SUBSTRING(@DS6, 37, 2) + SUBSTRING(@DS6, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS6, 33, 2) + SUBSTRING(@DS6, 31, 2) + SUBSTRING(@DS6, 29, 2) + SUBSTRING(@DS6, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS6, 25, 2) + SUBSTRING(@DS6, 23, 2) + SUBSTRING(@DS6, 21, 2) + SUBSTRING(@DS6, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS6, 17, 2) + SUBSTRING(@DS6, 15, 2) + SUBSTRING(@DS6, 13, 2) + SUBSTRING(@DS6, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS7 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D7)), 1) + '00000000000000000000000000000000', 42)
	SET @DS7 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS7, 41, 2) + SUBSTRING(@DS7, 39, 2) + SUBSTRING(@DS7, 37, 2) + SUBSTRING(@DS7, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS7, 33, 2) + SUBSTRING(@DS7, 31, 2) + SUBSTRING(@DS7, 29, 2) + SUBSTRING(@DS7, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS7, 25, 2) + SUBSTRING(@DS7, 23, 2) + SUBSTRING(@DS7, 21, 2) + SUBSTRING(@DS7, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS7, 17, 2) + SUBSTRING(@DS7, 15, 2) + SUBSTRING(@DS7, 13, 2) + SUBSTRING(@DS7, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS8 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D8)), 1) + '00000000000000000000000000000000', 42)
	SET @DS8 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS8, 41, 2) + SUBSTRING(@DS8, 39, 2) + SUBSTRING(@DS8, 37, 2) + SUBSTRING(@DS8, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS8, 33, 2) + SUBSTRING(@DS8, 31, 2) + SUBSTRING(@DS8, 29, 2) + SUBSTRING(@DS8, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS8, 25, 2) + SUBSTRING(@DS8, 23, 2) + SUBSTRING(@DS8, 21, 2) + SUBSTRING(@DS8, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS8, 17, 2) + SUBSTRING(@DS8, 15, 2) + SUBSTRING(@DS8, 13, 2) + SUBSTRING(@DS8, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	DECLARE @DS9 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @D9)), 1) + '00000000000000000000000000000000', 42)
	SET @DS9 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS9, 41, 2) + SUBSTRING(@DS9, 39, 2) + SUBSTRING(@DS9, 37, 2) + SUBSTRING(@DS9, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS9, 33, 2) + SUBSTRING(@DS9, 31, 2) + SUBSTRING(@DS9, 29, 2) + SUBSTRING(@DS9, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS9, 25, 2) + SUBSTRING(@DS9, 23, 2) + SUBSTRING(@DS9, 21, 2) + SUBSTRING(@DS9, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS9, 17, 2) + SUBSTRING(@DS9, 15, 2) + SUBSTRING(@DS9, 13, 2) + SUBSTRING(@DS9, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @RESULT = N'&[' + CHAR(10) + CHAR(13) + @DS0 + @DS1 + @DS2 + @DS3 + @DS4 + @DS5 + @DS6 + @DS7 + @DS8 + @DS9 + N'],' + CHAR(10) + CHAR(13)
	IF (LEN(@MULTIPLEXER) = 37)
		SELECT @RESULT

	IF LEN(@MULTIPLEXER) = 37
		BREAK

	SET @MULTIPLEXER = @MULTIPLEXER + N'0'

	SET @D0 = @D0 + N'0'
	SET @D1 = @D1 + N'0'
	SET @D2 = @D2 + N'0'
	SET @D3 = @D3 + N'0'
	SET @D4 = @D4 + N'0'
	SET @D5 = @D5 + N'0'
	SET @D6 = @D6 + N'0'
	SET @D7 = @D7 + N'0'
	SET @D8 = @D8 + N'0'
	SET @D9 = @D9 + N'0'

	--SELECT @MULTIPLEXER

END

--SELECT @RESULT

DECLARE @TEST AS DECIMAL(38, 38) = CAST('0.00000000000000000000000000000000000001' AS DECIMAL(38, 38))
SELECT @TEST

DECLARE @A AS DECIMAL(38, 2) = CAST('0.11' AS DECIMAL(38, 2))
DECLARE @B AS DECIMAL(38, 2) = CAST('1.10' AS DECIMAL(38, 2))
DECLARE @C AS DECIMAL(38, 2) = CAST('11.0' AS DECIMAL(38, 2))

--SELECT @A * @B, LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), @B), 1) + '00000000000000000000000000000000', 42) AS B, LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), @A * @B), 1) + '00000000000000000000000000000000', 42) AS AB
SELECT LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), @A), 1) + '00000000000000000000000000000000', 42) AS A
UNION ALL
SELECT LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), @B), 1) + '00000000000000000000000000000000', 42) AS A
UNION ALL
SELECT LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), @C), 1) + '00000000000000000000000000000000', 42) AS A

--0x26020001 0B000000000000000000000000000000 - 11
--0x26020001 6E000000000000000000000000000000 - 110
--0x26020001 4C040000000000000000000000000000 - 1100