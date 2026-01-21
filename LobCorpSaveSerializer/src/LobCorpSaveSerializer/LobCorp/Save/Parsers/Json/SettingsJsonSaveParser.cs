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

			data.CopyValue<float>(save, "bgmVolume");
			data.CopyValue<float>(save, "masterVolume");
			data.CopyValue<bool>(save, "tooltip");
			data.CopyValue<string>(save, "language");
			data.CopyValue<int>(save, "logIndex");

			return data;
		}
	}
}
