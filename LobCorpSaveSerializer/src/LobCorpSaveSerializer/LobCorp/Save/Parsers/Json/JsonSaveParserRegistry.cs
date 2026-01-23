using System.Collections.Generic;
using LobCorp.Save.Type;

namespace LobCorp.Save.Parsers.Json
{
	public static class JsonSaveParserRegistry
	{
		public static readonly Dictionary<SaveType, JsonSaveParserBase> ParsersMap = new Dictionary<SaveType, JsonSaveParserBase>()
		{
			{ SaveType.Options, new SettingsJsonSaveParser() },
			{ SaveType.Etc, new EtcJsonSaveParser() },
			{ SaveType.Master, new MasterJsonSaveParser() },
			{ SaveType.Global, new GlobalJsonSaveParser() },
			{ SaveType.Unlimited, new UnlimitedJsonSaveParser() },
			{ SaveType.WhiteNight, new WhiteNightJsonSaveParser() },
		};
	}
}
