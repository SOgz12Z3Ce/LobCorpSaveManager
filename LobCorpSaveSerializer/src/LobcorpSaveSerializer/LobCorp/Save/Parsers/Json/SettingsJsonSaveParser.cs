using System.Collections.Generic;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class SettingsJsonSaveParser : JsonSaveParserBase
	{
		public SettingsJsonSaveParser() : base(Type.SaveType.Settings)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var data = new Dictionary<string, object>();

			Copy<float>(data, save, "bgmVolume");
			Copy<float>(data, save, "bgmVolume");
			Copy<float>(data, save, "masterVolume");
			Copy<bool>(data, save, "tooltip");
			Copy<string>(data, save, "language");
			Copy<int>(data, save, "logIndex");

			return data;
		}
	}
}
