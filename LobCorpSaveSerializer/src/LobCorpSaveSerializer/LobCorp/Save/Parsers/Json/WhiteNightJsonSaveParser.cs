using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class WhiteNightJsonSaveParser : JsonSaveParserBase
	{
		public WhiteNightJsonSaveParser() : base(SaveType.WhiteNight)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var data = new Dictionary<string, object>();

			data.CopyValue<int>(save, "genDay");
			data.CopyValue<int>(save, "apostleListCount");
			data["apostleList"] = ParseApostles(save["apostleList"] as JObject);
			
			return data;
		}
		private static Dictionary<int, Dictionary<string, object>> ParseApostles(JObject ApostleList)
		{
			var result = new Dictionary<int, Dictionary<string, object>>();
			foreach (var kvp in ApostleList)
			{
				result[int.Parse(kvp.Key)] = ParseApostle(kvp.Value as JObject);
			}
			return result;
		}
		private static Dictionary<string, object> ParseApostle(JObject apostleListItem)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<int>(apostleListItem, "NameId");
			result["hairData"] = JsonSaveParserUtils.ParseStyle(apostleListItem["hairData"] as JObject);
			result["HairColor"] = JsonSaveParserUtils.ParseColor(apostleListItem["HairColor"] as JObject);
			result.CopyValue<long>(apostleListItem, "isntId");
			result.CopyValue<string>(apostleListItem, "Name");

			return result;
		}
	}
}
