-- RUN ONE
--
	DECLARE @S AS NVARCHAR(255)
	SET @S = '1'
	DECLARE @DS0 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS0 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS0, 41, 2) + SUBSTRING(@DS0, 39, 2) + SUBSTRING(@DS0, 37, 2) + SUBSTRING(@DS0, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS0, 33, 2) + SUBSTRING(@DS0, 31, 2) + SUBSTRING(@DS0, 29, 2) + SUBSTRING(@DS0, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS0, 25, 2) + SUBSTRING(@DS0, 23, 2) + SUBSTRING(@DS0, 21, 2) + SUBSTRING(@DS0, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS0, 17, 2) + SUBSTRING(@DS0, 15, 2) + SUBSTRING(@DS0, 13, 2) + SUBSTRING(@DS0, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS1 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS1 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS1, 41, 2) + SUBSTRING(@DS1, 39, 2) + SUBSTRING(@DS1, 37, 2) + SUBSTRING(@DS1, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS1, 33, 2) + SUBSTRING(@DS1, 31, 2) + SUBSTRING(@DS1, 29, 2) + SUBSTRING(@DS1, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS1, 25, 2) + SUBSTRING(@DS1, 23, 2) + SUBSTRING(@DS1, 21, 2) + SUBSTRING(@DS1, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS1, 17, 2) + SUBSTRING(@DS1, 15, 2) + SUBSTRING(@DS1, 13, 2) + SUBSTRING(@DS1, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS2 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS2 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS2, 41, 2) + SUBSTRING(@DS2, 39, 2) + SUBSTRING(@DS2, 37, 2) + SUBSTRING(@DS2, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS2, 33, 2) + SUBSTRING(@DS2, 31, 2) + SUBSTRING(@DS2, 29, 2) + SUBSTRING(@DS2, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS2, 25, 2) + SUBSTRING(@DS2, 23, 2) + SUBSTRING(@DS2, 21, 2) + SUBSTRING(@DS2, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS2, 17, 2) + SUBSTRING(@DS2, 15, 2) + SUBSTRING(@DS2, 13, 2) + SUBSTRING(@DS2, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS3 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS3 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS3, 41, 2) + SUBSTRING(@DS3, 39, 2) + SUBSTRING(@DS3, 37, 2) + SUBSTRING(@DS3, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS3, 33, 2) + SUBSTRING(@DS3, 31, 2) + SUBSTRING(@DS3, 29, 2) + SUBSTRING(@DS3, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS3, 25, 2) + SUBSTRING(@DS3, 23, 2) + SUBSTRING(@DS3, 21, 2) + SUBSTRING(@DS3, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS3, 17, 2) + SUBSTRING(@DS3, 15, 2) + SUBSTRING(@DS3, 13, 2) + SUBSTRING(@DS3, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS4 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS4 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS4, 41, 2) + SUBSTRING(@DS4, 39, 2) + SUBSTRING(@DS4, 37, 2) + SUBSTRING(@DS4, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS4, 33, 2) + SUBSTRING(@DS4, 31, 2) + SUBSTRING(@DS4, 29, 2) + SUBSTRING(@DS4, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS4, 25, 2) + SUBSTRING(@DS4, 23, 2) + SUBSTRING(@DS4, 21, 2) + SUBSTRING(@DS4, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS4, 17, 2) + SUBSTRING(@DS4, 15, 2) + SUBSTRING(@DS4, 13, 2) + SUBSTRING(@DS4, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS5 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS5 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS5, 41, 2) + SUBSTRING(@DS5, 39, 2) + SUBSTRING(@DS5, 37, 2) + SUBSTRING(@DS5, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS5, 33, 2) + SUBSTRING(@DS5, 31, 2) + SUBSTRING(@DS5, 29, 2) + SUBSTRING(@DS5, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS5, 25, 2) + SUBSTRING(@DS5, 23, 2) + SUBSTRING(@DS5, 21, 2) + SUBSTRING(@DS5, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS5, 17, 2) + SUBSTRING(@DS5, 15, 2) + SUBSTRING(@DS5, 13, 2) + SUBSTRING(@DS5, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS6 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS6 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS6, 41, 2) + SUBSTRING(@DS6, 39, 2) + SUBSTRING(@DS6, 37, 2) + SUBSTRING(@DS6, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS6, 33, 2) + SUBSTRING(@DS6, 31, 2) + SUBSTRING(@DS6, 29, 2) + SUBSTRING(@DS6, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS6, 25, 2) + SUBSTRING(@DS6, 23, 2) + SUBSTRING(@DS6, 21, 2) + SUBSTRING(@DS6, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS6, 17, 2) + SUBSTRING(@DS6, 15, 2) + SUBSTRING(@DS6, 13, 2) + SUBSTRING(@DS6, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS7 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS7 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS7, 41, 2) + SUBSTRING(@DS7, 39, 2) + SUBSTRING(@DS7, 37, 2) + SUBSTRING(@DS7, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS7, 33, 2) + SUBSTRING(@DS7, 31, 2) + SUBSTRING(@DS7, 29, 2) + SUBSTRING(@DS7, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS7, 25, 2) + SUBSTRING(@DS7, 23, 2) + SUBSTRING(@DS7, 21, 2) + SUBSTRING(@DS7, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS7, 17, 2) + SUBSTRING(@DS7, 15, 2) + SUBSTRING(@DS7, 13, 2) + SUBSTRING(@DS7, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS8 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS8 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS8, 41, 2) + SUBSTRING(@DS8, 39, 2) + SUBSTRING(@DS8, 37, 2) + SUBSTRING(@DS8, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS8, 33, 2) + SUBSTRING(@DS8, 31, 2) + SUBSTRING(@DS8, 29, 2) + SUBSTRING(@DS8, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS8, 25, 2) + SUBSTRING(@DS8, 23, 2) + SUBSTRING(@DS8, 21, 2) + SUBSTRING(@DS8, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS8, 17, 2) + SUBSTRING(@DS8, 15, 2) + SUBSTRING(@DS8, 13, 2) + SUBSTRING(@DS8, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS9 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS9 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS9, 41, 2) + SUBSTRING(@DS9, 39, 2) + SUBSTRING(@DS9, 37, 2) + SUBSTRING(@DS9, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS9, 33, 2) + SUBSTRING(@DS9, 31, 2) + SUBSTRING(@DS9, 29, 2) + SUBSTRING(@DS9, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS9, 25, 2) + SUBSTRING(@DS9, 23, 2) + SUBSTRING(@DS9, 21, 2) + SUBSTRING(@DS9, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS9, 17, 2) + SUBSTRING(@DS9, 15, 2) + SUBSTRING(@DS9, 13, 2) + SUBSTRING(@DS9, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS10 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS10 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS10, 41, 2) + SUBSTRING(@DS10, 39, 2) + SUBSTRING(@DS10, 37, 2) + SUBSTRING(@DS10, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS10, 33, 2) + SUBSTRING(@DS10, 31, 2) + SUBSTRING(@DS10, 29, 2) + SUBSTRING(@DS10, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS10, 25, 2) + SUBSTRING(@DS10, 23, 2) + SUBSTRING(@DS10, 21, 2) + SUBSTRING(@DS10, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS10, 17, 2) + SUBSTRING(@DS10, 15, 2) + SUBSTRING(@DS10, 13, 2) + SUBSTRING(@DS10, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS11 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS11 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS11, 41, 2) + SUBSTRING(@DS11, 39, 2) + SUBSTRING(@DS11, 37, 2) + SUBSTRING(@DS11, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS11, 33, 2) + SUBSTRING(@DS11, 31, 2) + SUBSTRING(@DS11, 29, 2) + SUBSTRING(@DS11, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS11, 25, 2) + SUBSTRING(@DS11, 23, 2) + SUBSTRING(@DS11, 21, 2) + SUBSTRING(@DS11, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS11, 17, 2) + SUBSTRING(@DS11, 15, 2) + SUBSTRING(@DS11, 13, 2) + SUBSTRING(@DS11, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS12 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS12 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS12, 41, 2) + SUBSTRING(@DS12, 39, 2) + SUBSTRING(@DS12, 37, 2) + SUBSTRING(@DS12, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS12, 33, 2) + SUBSTRING(@DS12, 31, 2) + SUBSTRING(@DS12, 29, 2) + SUBSTRING(@DS12, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS12, 25, 2) + SUBSTRING(@DS12, 23, 2) + SUBSTRING(@DS12, 21, 2) + SUBSTRING(@DS12, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS12, 17, 2) + SUBSTRING(@DS12, 15, 2) + SUBSTRING(@DS12, 13, 2) + SUBSTRING(@DS12, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS13 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS13 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS13, 41, 2) + SUBSTRING(@DS13, 39, 2) + SUBSTRING(@DS13, 37, 2) + SUBSTRING(@DS13, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS13, 33, 2) + SUBSTRING(@DS13, 31, 2) + SUBSTRING(@DS13, 29, 2) + SUBSTRING(@DS13, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS13, 25, 2) + SUBSTRING(@DS13, 23, 2) + SUBSTRING(@DS13, 21, 2) + SUBSTRING(@DS13, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS13, 17, 2) + SUBSTRING(@DS13, 15, 2) + SUBSTRING(@DS13, 13, 2) + SUBSTRING(@DS13, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS14 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS14 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS14, 41, 2) + SUBSTRING(@DS14, 39, 2) + SUBSTRING(@DS14, 37, 2) + SUBSTRING(@DS14, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS14, 33, 2) + SUBSTRING(@DS14, 31, 2) + SUBSTRING(@DS14, 29, 2) + SUBSTRING(@DS14, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS14, 25, 2) + SUBSTRING(@DS14, 23, 2) + SUBSTRING(@DS14, 21, 2) + SUBSTRING(@DS14, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS14, 17, 2) + SUBSTRING(@DS14, 15, 2) + SUBSTRING(@DS14, 13, 2) + SUBSTRING(@DS14, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS15 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS15 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS15, 41, 2) + SUBSTRING(@DS15, 39, 2) + SUBSTRING(@DS15, 37, 2) + SUBSTRING(@DS15, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS15, 33, 2) + SUBSTRING(@DS15, 31, 2) + SUBSTRING(@DS15, 29, 2) + SUBSTRING(@DS15, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS15, 25, 2) + SUBSTRING(@DS15, 23, 2) + SUBSTRING(@DS15, 21, 2) + SUBSTRING(@DS15, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS15, 17, 2) + SUBSTRING(@DS15, 15, 2) + SUBSTRING(@DS15, 13, 2) + SUBSTRING(@DS15, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS16 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS16 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS16, 41, 2) + SUBSTRING(@DS16, 39, 2) + SUBSTRING(@DS16, 37, 2) + SUBSTRING(@DS16, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS16, 33, 2) + SUBSTRING(@DS16, 31, 2) + SUBSTRING(@DS16, 29, 2) + SUBSTRING(@DS16, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS16, 25, 2) + SUBSTRING(@DS16, 23, 2) + SUBSTRING(@DS16, 21, 2) + SUBSTRING(@DS16, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS16, 17, 2) + SUBSTRING(@DS16, 15, 2) + SUBSTRING(@DS16, 13, 2) + SUBSTRING(@DS16, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS17 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS17 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS17, 41, 2) + SUBSTRING(@DS17, 39, 2) + SUBSTRING(@DS17, 37, 2) + SUBSTRING(@DS17, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS17, 33, 2) + SUBSTRING(@DS17, 31, 2) + SUBSTRING(@DS17, 29, 2) + SUBSTRING(@DS17, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS17, 25, 2) + SUBSTRING(@DS17, 23, 2) + SUBSTRING(@DS17, 21, 2) + SUBSTRING(@DS17, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS17, 17, 2) + SUBSTRING(@DS17, 15, 2) + SUBSTRING(@DS17, 13, 2) + SUBSTRING(@DS17, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS18 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS18 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS18, 41, 2) + SUBSTRING(@DS18, 39, 2) + SUBSTRING(@DS18, 37, 2) + SUBSTRING(@DS18, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS18, 33, 2) + SUBSTRING(@DS18, 31, 2) + SUBSTRING(@DS18, 29, 2) + SUBSTRING(@DS18, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS18, 25, 2) + SUBSTRING(@DS18, 23, 2) + SUBSTRING(@DS18, 21, 2) + SUBSTRING(@DS18, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS18, 17, 2) + SUBSTRING(@DS18, 15, 2) + SUBSTRING(@DS18, 13, 2) + SUBSTRING(@DS18, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS19 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS19 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS19, 41, 2) + SUBSTRING(@DS19, 39, 2) + SUBSTRING(@DS19, 37, 2) + SUBSTRING(@DS19, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS19, 33, 2) + SUBSTRING(@DS19, 31, 2) + SUBSTRING(@DS19, 29, 2) + SUBSTRING(@DS19, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS19, 25, 2) + SUBSTRING(@DS19, 23, 2) + SUBSTRING(@DS19, 21, 2) + SUBSTRING(@DS19, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS19, 17, 2) + SUBSTRING(@DS19, 15, 2) + SUBSTRING(@DS19, 13, 2) + SUBSTRING(@DS19, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS20 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS20 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS20, 41, 2) + SUBSTRING(@DS20, 39, 2) + SUBSTRING(@DS20, 37, 2) + SUBSTRING(@DS20, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS20, 33, 2) + SUBSTRING(@DS20, 31, 2) + SUBSTRING(@DS20, 29, 2) + SUBSTRING(@DS20, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS20, 25, 2) + SUBSTRING(@DS20, 23, 2) + SUBSTRING(@DS20, 21, 2) + SUBSTRING(@DS20, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS20, 17, 2) + SUBSTRING(@DS20, 15, 2) + SUBSTRING(@DS20, 13, 2) + SUBSTRING(@DS20, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS21 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS21 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS21, 41, 2) + SUBSTRING(@DS21, 39, 2) + SUBSTRING(@DS21, 37, 2) + SUBSTRING(@DS21, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS21, 33, 2) + SUBSTRING(@DS21, 31, 2) + SUBSTRING(@DS21, 29, 2) + SUBSTRING(@DS21, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS21, 25, 2) + SUBSTRING(@DS21, 23, 2) + SUBSTRING(@DS21, 21, 2) + SUBSTRING(@DS21, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS21, 17, 2) + SUBSTRING(@DS21, 15, 2) + SUBSTRING(@DS21, 13, 2) + SUBSTRING(@DS21, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS22 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS22 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS22, 41, 2) + SUBSTRING(@DS22, 39, 2) + SUBSTRING(@DS22, 37, 2) + SUBSTRING(@DS22, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS22, 33, 2) + SUBSTRING(@DS22, 31, 2) + SUBSTRING(@DS22, 29, 2) + SUBSTRING(@DS22, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS22, 25, 2) + SUBSTRING(@DS22, 23, 2) + SUBSTRING(@DS22, 21, 2) + SUBSTRING(@DS22, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS22, 17, 2) + SUBSTRING(@DS22, 15, 2) + SUBSTRING(@DS22, 13, 2) + SUBSTRING(@DS22, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS23 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS23 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS23, 41, 2) + SUBSTRING(@DS23, 39, 2) + SUBSTRING(@DS23, 37, 2) + SUBSTRING(@DS23, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS23, 33, 2) + SUBSTRING(@DS23, 31, 2) + SUBSTRING(@DS23, 29, 2) + SUBSTRING(@DS23, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS23, 25, 2) + SUBSTRING(@DS23, 23, 2) + SUBSTRING(@DS23, 21, 2) + SUBSTRING(@DS23, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS23, 17, 2) + SUBSTRING(@DS23, 15, 2) + SUBSTRING(@DS23, 13, 2) + SUBSTRING(@DS23, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS24 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS24 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS24, 41, 2) + SUBSTRING(@DS24, 39, 2) + SUBSTRING(@DS24, 37, 2) + SUBSTRING(@DS24, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS24, 33, 2) + SUBSTRING(@DS24, 31, 2) + SUBSTRING(@DS24, 29, 2) + SUBSTRING(@DS24, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS24, 25, 2) + SUBSTRING(@DS24, 23, 2) + SUBSTRING(@DS24, 21, 2) + SUBSTRING(@DS24, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS24, 17, 2) + SUBSTRING(@DS24, 15, 2) + SUBSTRING(@DS24, 13, 2) + SUBSTRING(@DS24, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS25 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS25 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS25, 41, 2) + SUBSTRING(@DS25, 39, 2) + SUBSTRING(@DS25, 37, 2) + SUBSTRING(@DS25, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS25, 33, 2) + SUBSTRING(@DS25, 31, 2) + SUBSTRING(@DS25, 29, 2) + SUBSTRING(@DS25, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS25, 25, 2) + SUBSTRING(@DS25, 23, 2) + SUBSTRING(@DS25, 21, 2) + SUBSTRING(@DS25, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS25, 17, 2) + SUBSTRING(@DS25, 15, 2) + SUBSTRING(@DS25, 13, 2) + SUBSTRING(@DS25, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS26 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS26 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS26, 41, 2) + SUBSTRING(@DS26, 39, 2) + SUBSTRING(@DS26, 37, 2) + SUBSTRING(@DS26, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS26, 33, 2) + SUBSTRING(@DS26, 31, 2) + SUBSTRING(@DS26, 29, 2) + SUBSTRING(@DS26, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS26, 25, 2) + SUBSTRING(@DS26, 23, 2) + SUBSTRING(@DS26, 21, 2) + SUBSTRING(@DS26, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS26, 17, 2) + SUBSTRING(@DS26, 15, 2) + SUBSTRING(@DS26, 13, 2) + SUBSTRING(@DS26, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS27 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS27 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS27, 41, 2) + SUBSTRING(@DS27, 39, 2) + SUBSTRING(@DS27, 37, 2) + SUBSTRING(@DS27, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS27, 33, 2) + SUBSTRING(@DS27, 31, 2) + SUBSTRING(@DS27, 29, 2) + SUBSTRING(@DS27, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS27, 25, 2) + SUBSTRING(@DS27, 23, 2) + SUBSTRING(@DS27, 21, 2) + SUBSTRING(@DS27, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS27, 17, 2) + SUBSTRING(@DS27, 15, 2) + SUBSTRING(@DS27, 13, 2) + SUBSTRING(@DS27, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS28 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS28 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS28, 41, 2) + SUBSTRING(@DS28, 39, 2) + SUBSTRING(@DS28, 37, 2) + SUBSTRING(@DS28, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS28, 33, 2) + SUBSTRING(@DS28, 31, 2) + SUBSTRING(@DS28, 29, 2) + SUBSTRING(@DS28, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS28, 25, 2) + SUBSTRING(@DS28, 23, 2) + SUBSTRING(@DS28, 21, 2) + SUBSTRING(@DS28, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS28, 17, 2) + SUBSTRING(@DS28, 15, 2) + SUBSTRING(@DS28, 13, 2) + SUBSTRING(@DS28, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS29 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS29 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS29, 41, 2) + SUBSTRING(@DS29, 39, 2) + SUBSTRING(@DS29, 37, 2) + SUBSTRING(@DS29, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS29, 33, 2) + SUBSTRING(@DS29, 31, 2) + SUBSTRING(@DS29, 29, 2) + SUBSTRING(@DS29, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS29, 25, 2) + SUBSTRING(@DS29, 23, 2) + SUBSTRING(@DS29, 21, 2) + SUBSTRING(@DS29, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS29, 17, 2) + SUBSTRING(@DS29, 15, 2) + SUBSTRING(@DS29, 13, 2) + SUBSTRING(@DS29, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS30 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS30 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS30, 41, 2) + SUBSTRING(@DS30, 39, 2) + SUBSTRING(@DS30, 37, 2) + SUBSTRING(@DS30, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS30, 33, 2) + SUBSTRING(@DS30, 31, 2) + SUBSTRING(@DS30, 29, 2) + SUBSTRING(@DS30, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS30, 25, 2) + SUBSTRING(@DS30, 23, 2) + SUBSTRING(@DS30, 21, 2) + SUBSTRING(@DS30, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS30, 17, 2) + SUBSTRING(@DS30, 15, 2) + SUBSTRING(@DS30, 13, 2) + SUBSTRING(@DS30, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS31 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS31 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS31, 41, 2) + SUBSTRING(@DS31, 39, 2) + SUBSTRING(@DS31, 37, 2) + SUBSTRING(@DS31, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS31, 33, 2) + SUBSTRING(@DS31, 31, 2) + SUBSTRING(@DS31, 29, 2) + SUBSTRING(@DS31, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS31, 25, 2) + SUBSTRING(@DS31, 23, 2) + SUBSTRING(@DS31, 21, 2) + SUBSTRING(@DS31, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS31, 17, 2) + SUBSTRING(@DS31, 15, 2) + SUBSTRING(@DS31, 13, 2) + SUBSTRING(@DS31, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS32 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS32 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS32, 41, 2) + SUBSTRING(@DS32, 39, 2) + SUBSTRING(@DS32, 37, 2) + SUBSTRING(@DS32, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS32, 33, 2) + SUBSTRING(@DS32, 31, 2) + SUBSTRING(@DS32, 29, 2) + SUBSTRING(@DS32, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS32, 25, 2) + SUBSTRING(@DS32, 23, 2) + SUBSTRING(@DS32, 21, 2) + SUBSTRING(@DS32, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS32, 17, 2) + SUBSTRING(@DS32, 15, 2) + SUBSTRING(@DS32, 13, 2) + SUBSTRING(@DS32, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS33 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS33 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS33, 41, 2) + SUBSTRING(@DS33, 39, 2) + SUBSTRING(@DS33, 37, 2) + SUBSTRING(@DS33, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS33, 33, 2) + SUBSTRING(@DS33, 31, 2) + SUBSTRING(@DS33, 29, 2) + SUBSTRING(@DS33, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS33, 25, 2) + SUBSTRING(@DS33, 23, 2) + SUBSTRING(@DS33, 21, 2) + SUBSTRING(@DS33, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS33, 17, 2) + SUBSTRING(@DS33, 15, 2) + SUBSTRING(@DS33, 13, 2) + SUBSTRING(@DS33, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS34 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS34 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS34, 41, 2) + SUBSTRING(@DS34, 39, 2) + SUBSTRING(@DS34, 37, 2) + SUBSTRING(@DS34, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS34, 33, 2) + SUBSTRING(@DS34, 31, 2) + SUBSTRING(@DS34, 29, 2) + SUBSTRING(@DS34, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS34, 25, 2) + SUBSTRING(@DS34, 23, 2) + SUBSTRING(@DS34, 21, 2) + SUBSTRING(@DS34, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS34, 17, 2) + SUBSTRING(@DS34, 15, 2) + SUBSTRING(@DS34, 13, 2) + SUBSTRING(@DS34, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS35 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS35 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS35, 41, 2) + SUBSTRING(@DS35, 39, 2) + SUBSTRING(@DS35, 37, 2) + SUBSTRING(@DS35, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS35, 33, 2) + SUBSTRING(@DS35, 31, 2) + SUBSTRING(@DS35, 29, 2) + SUBSTRING(@DS35, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS35, 25, 2) + SUBSTRING(@DS35, 23, 2) + SUBSTRING(@DS35, 21, 2) + SUBSTRING(@DS35, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS35, 17, 2) + SUBSTRING(@DS35, 15, 2) + SUBSTRING(@DS35, 13, 2) + SUBSTRING(@DS35, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS36 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS36 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS36, 41, 2) + SUBSTRING(@DS36, 39, 2) + SUBSTRING(@DS36, 37, 2) + SUBSTRING(@DS36, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS36, 33, 2) + SUBSTRING(@DS36, 31, 2) + SUBSTRING(@DS36, 29, 2) + SUBSTRING(@DS36, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS36, 25, 2) + SUBSTRING(@DS36, 23, 2) + SUBSTRING(@DS36, 21, 2) + SUBSTRING(@DS36, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS36, 17, 2) + SUBSTRING(@DS36, 15, 2) + SUBSTRING(@DS36, 13, 2) + SUBSTRING(@DS36, 11, 2) + ',},' + CHAR(10) + CHAR(13)

	SET @S = @S +'0'
	DECLARE @DS37 AS NVARCHAR(4000) = LEFT(CONVERT(NVARCHAR(255), CONVERT(VARBINARY(32), CONVERT(DECIMAL(38, 0), @S)), 1) + '00000000000000000000000000000000', 42)
	SET @DS37 = ''
		+ CHAR(9) + N'Decimal { sign: DecimalSign::Positive, precision: DecimalPrecision::Precision38, scale: DecimalScale::Scale00, '
		+ N'part_3: 0x' + SUBSTRING(@DS37, 41, 2) + SUBSTRING(@DS37, 39, 2) + SUBSTRING(@DS37, 37, 2) + SUBSTRING(@DS37, 35, 2) + ', '
		+ N'part_2: 0x' + SUBSTRING(@DS37, 33, 2) + SUBSTRING(@DS37, 31, 2) + SUBSTRING(@DS37, 29, 2) + SUBSTRING(@DS37, 27, 2) + ', '
		+ N'part_1: 0x' + SUBSTRING(@DS37, 25, 2) + SUBSTRING(@DS37, 23, 2) + SUBSTRING(@DS37, 21, 2) + SUBSTRING(@DS37, 19, 2) + ', '
		+ N'part_0: 0x' + SUBSTRING(@DS37, 17, 2) + SUBSTRING(@DS37, 15, 2) + SUBSTRING(@DS37, 13, 2) + SUBSTRING(@DS37, 11, 2) + ',},' + CHAR(10) + CHAR(13)

SELECT @DS0 + @DS1 + @DS2 + @DS3 + @DS4 + @DS5 + @DS6 + @DS7 + @DS8 + @DS9 AS F0
SELECT @DS10 + @DS11 + @DS12 + @DS13 + @DS14 + @DS15 + @DS16 + @DS17 + @DS18 + @DS19 AS F0
SELECT @DS20 + @DS21 + @DS22 + @DS23 + @DS24 + @DS25 + @DS26 + @DS27 + @DS28 + @DS29 AS F0
SELECT @DS30 + @DS31 + @DS32 + @DS33 + @DS34 + @DS35 + @DS36 + @DS37 AS F0
