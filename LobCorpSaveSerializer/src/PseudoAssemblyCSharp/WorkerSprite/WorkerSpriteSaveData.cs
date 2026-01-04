using System;

namespace WorkerSprite
{
	[Serializable]
	public class WorkerSpriteSaveData
	{
		public bool bInit;
		public Pair Eye;
		public Pair EyeClose;
		public Pair EyePanic;
		public Pair EyeDead;
		public Pair EyeBrow;
		public Pair BattleEyeBrow;
		public Pair PanicEyeBrow;
		public ColorData EyeColor;
		public Pair Mouth;
		public Pair BattleMouth;
		public Pair PanicMouth;
		public Pair FrontHair;
		public Pair RearHair;
		public Pair AttachmentHair;
		public ColorData HairColor;
		[Serializable]
		public class Pair
		{
			public long a;
			public int b;
		}
		[Serializable]
		public class ColorData
		{
			public float r;
			public float g;
			public float b;
			public float a;
		}
	}
}
