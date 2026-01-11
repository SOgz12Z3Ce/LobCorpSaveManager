using System;
using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class UnlimitedJsonSaveParser : JsonSaveParserBase
	{
		public UnlimitedJsonSaveParser() : base(SaveType.Unlimited)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<string>(save, "saveVer");
			result["money"] = JsonSaveParserUtils.ParseLobPoint(save["money"] as JObject);
			result["agents"] = JsonSaveParserUtils.ParseAgent(save["agents"] as JObject);
			result["creatures"] = JsonSaveParserUtils.ParseAbnormality(save["creatures"] as JObject);
			return result;
		}
	}
}
